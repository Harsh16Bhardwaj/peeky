use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::{Path, PathBuf},
};

use chrono::{Duration as ChronoDuration, Local};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::ActivitySettings;

pub const SESSION_TARGET_SECS: f64 = 7_200.0;
pub const REVIEW_THRESHOLD_SECS: f64 = 180.0;
const DATABASE_SCHEMA_VERSION: i64 = 2;

#[derive(Debug, Error)]
pub enum ActivityError {
    #[error("Activity database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Activity storage error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Activity data error: {0}")]
    Data(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ActivityCategory {
    Productive,
    Neutral,
    Distraction,
    Break,
}

impl ActivityCategory {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Productive => "productive",
            Self::Neutral => "neutral",
            Self::Distraction => "distraction",
            Self::Break => "break",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "productive" => Some(Self::Productive),
            "neutral" => Some(Self::Neutral),
            "distraction" => Some(Self::Distraction),
            "break" => Some(Self::Break),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ActivitySourceKind {
    Application,
    Browser,
    System,
}

impl ActivitySourceKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Application => "application",
            Self::Browser => "browser",
            Self::System => "system",
        }
    }

    fn parse(value: &str) -> Self {
        match value {
            "browser" => Self::Browser,
            "system" => Self::System,
            _ => Self::Application,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySourceInput {
    pub kind: ActivitySourceKind,
    pub executable: String,
    pub display_name: String,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub audible: bool,
}

impl ActivitySourceInput {
    fn memory_key(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.kind.as_str(),
            self.executable.to_ascii_lowercase(),
            self.domain
                .as_deref()
                .unwrap_or_default()
                .to_ascii_lowercase(),
            self.title.as_deref().unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySource {
    pub id: i64,
    pub kind: ActivitySourceKind,
    pub executable: Option<String>,
    pub name: String,
    pub domain: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySegment {
    pub id: i64,
    pub session_id: String,
    pub source_id: Option<i64>,
    pub started_at_epoch_ms: i64,
    pub ended_at_epoch_ms: i64,
    pub duration_secs: f64,
    pub credited_secs: f64,
    pub bucket: String,
    pub category: Option<ActivityCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityAggregate {
    pub source: ActivitySource,
    pub duration_secs: f64,
    pub category: Option<ActivityCategory>,
    pub qualifying: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySession {
    pub id: String,
    pub local_date: String,
    pub started_at_epoch_ms: i64,
    pub ended_at_epoch_ms: Option<i64>,
    pub active_secs: f64,
    pub status: String,
    pub review_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionReview {
    pub session: ActivitySession,
    pub activities: Vec<ActivityAggregate>,
    pub timeline: Vec<ActivitySegment>,
    pub category_totals: BTreeMap<String, f64>,
    pub short_activity_secs: f64,
    pub short_switch_count: u64,
    pub pending_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionClassification {
    pub source_id: i64,
    pub category: ActivityCategory,
    pub use_next_time: bool,
    pub domain_wide: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyActivitySummary {
    pub local_date: String,
    pub category_totals: BTreeMap<String, f64>,
    pub completed_sessions: u64,
    pub partial_sessions: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDashboard {
    pub range_days: u32,
    pub sessions: Vec<ActivitySession>,
    pub activities: Vec<ActivityAggregate>,
    pub category_totals: BTreeMap<String, f64>,
    pub daily: Vec<DailyActivitySummary>,
    pub pending_reviews: u64,
    pub active_secs: f64,
    pub break_secs: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationRule {
    pub id: i64,
    pub source_kind: ActivitySourceKind,
    pub matcher: String,
    pub display_name: String,
    pub category: ActivityCategory,
    pub domain_wide: bool,
    pub created_at_epoch_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingStatus {
    pub consented: bool,
    pub enabled: bool,
    pub paused: bool,
    pub status: String,
    pub current_session_id: Option<String>,
    pub current_session_active_secs: f64,
    pub session_target_secs: f64,
    pub pending_reviews: u64,
}

#[derive(Debug, Clone)]
pub struct ActivityTick {
    pub now_epoch_ms: i64,
    pub local_date: String,
    pub delta_secs: f64,
    pub idle_secs: u64,
    pub locked_or_sleeping: bool,
    pub break_active: bool,
    pub source: Option<ActivitySourceInput>,
}

#[derive(Debug, Clone)]
pub enum ActivityEvent {
    SessionCompleted(ActivitySession),
    SessionChanged,
    TrackingStatusChanged,
}

#[derive(Debug, Clone)]
pub struct ActivityRepository {
    path: PathBuf,
}

#[derive(Debug, Clone)]
struct ResolvedSource {
    id: i64,
    identity_key: String,
    classification: Option<ActivityCategory>,
}

impl ActivityRepository {
    pub fn initialize(path: impl Into<PathBuf>) -> Result<Self, ActivityError> {
        let path = path.into();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let existing_version = if path.exists() {
            let connection = Connection::open(&path)?;
            connection.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))?
        } else {
            0
        };
        if path.exists() && existing_version < DATABASE_SCHEMA_VERSION {
            let backup = path.with_extension(format!(
                "pre-v{}-{}.db",
                DATABASE_SCHEMA_VERSION,
                Local::now().format("%Y%m%d-%H%M%S")
            ));
            fs::copy(&path, backup)?;
        }

        let repository = Self { path };
        repository.migrate()?;
        Ok(repository)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn connect(&self) -> Result<Connection, ActivityError> {
        let connection = Connection::open(&self.path)?;
        connection.busy_timeout(std::time::Duration::from_secs(3))?;
        // Do not renegotiate journal mode on every short-lived read/write connection.
        // On some Windows profiles that turns a healthy database into SQLITE_IOERR;
        // the existing journal mode remains persisted by SQLite.
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.pragma_update(None, "synchronous", "NORMAL")?;
        Ok(connection)
    }

    fn migrate(&self) -> Result<(), ActivityError> {
        let mut connection = self.connect()?;
        let tx = connection.transaction()?;
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS activity_metadata (
                key TEXT PRIMARY KEY,
                value BLOB NOT NULL
             );
             CREATE TABLE IF NOT EXISTS activity_sessions (
                id TEXT PRIMARY KEY,
                local_date TEXT NOT NULL,
                started_at_ms INTEGER NOT NULL,
                ended_at_ms INTEGER,
                active_secs REAL NOT NULL DEFAULT 0,
                status TEXT NOT NULL,
                review_status TEXT NOT NULL DEFAULT 'pending'
             );
             CREATE INDEX IF NOT EXISTS idx_sessions_date ON activity_sessions(local_date);
             CREATE TABLE IF NOT EXISTS activity_sources (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                kind TEXT NOT NULL,
                identity_key TEXT NOT NULL UNIQUE,
                executable TEXT,
                display_name TEXT NOT NULL,
                domain TEXT,
                encrypted_title BLOB,
                title_hash TEXT
             );
             CREATE TABLE IF NOT EXISTS activity_segments (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL REFERENCES activity_sessions(id) ON DELETE CASCADE,
                source_id INTEGER REFERENCES activity_sources(id) ON DELETE SET NULL,
                started_at_ms INTEGER NOT NULL,
                ended_at_ms INTEGER NOT NULL,
                duration_secs REAL NOT NULL,
                credited_secs REAL NOT NULL,
                bucket TEXT NOT NULL,
                classification TEXT
             );
             CREATE INDEX IF NOT EXISTS idx_segments_session ON activity_segments(session_id, started_at_ms);
             CREATE INDEX IF NOT EXISTS idx_segments_source ON activity_segments(source_id);
             CREATE TABLE IF NOT EXISTS classification_rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_kind TEXT NOT NULL,
                matcher TEXT NOT NULL UNIQUE,
                display_name TEXT NOT NULL,
                category TEXT NOT NULL,
                domain_wide INTEGER NOT NULL DEFAULT 0,
                created_at_ms INTEGER NOT NULL
             );
             ",
        )?;
        let version = tx.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))?;
        if version < 2 {
            tx.execute_batch(
                "INSERT INTO activity_sources(kind, identity_key, executable, display_name)
                 VALUES('application', 'app:chrome.exe', 'chrome.exe', 'Google Chrome')
                 ON CONFLICT(identity_key) DO UPDATE SET
                   kind='application', executable='chrome.exe', display_name='Google Chrome',
                   domain=NULL, encrypted_title=NULL, title_hash=NULL;
                 UPDATE activity_segments
                 SET source_id=(SELECT id FROM activity_sources WHERE identity_key='app:chrome.exe')
                 WHERE source_id IN (
                   SELECT id FROM activity_sources
                   WHERE kind='browser' AND lower(executable)='chrome.exe'
                 );
                 DELETE FROM classification_rules WHERE source_kind='browser';
                 DELETE FROM activity_sources WHERE kind='browser';
                 PRAGMA user_version = 2;",
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn set_tracking_paused(&self, paused: bool) -> Result<(), ActivityError> {
        let connection = self.connect()?;
        connection.execute(
            "INSERT INTO activity_metadata(key, value) VALUES('tracking_paused', ?1)
             ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![if paused {
                b"1".as_slice()
            } else {
                b"0".as_slice()
            }],
        )?;
        Ok(())
    }

    pub fn tracking_paused(&self) -> Result<bool, ActivityError> {
        let connection = self.connect()?;
        let value = connection
            .query_row(
                "SELECT value FROM activity_metadata WHERE key='tracking_paused'",
                [],
                |row| row.get::<_, Vec<u8>>(0),
            )
            .optional()?;
        Ok(value.as_deref() == Some(b"1"))
    }

    pub fn active_session(&self) -> Result<Option<ActivitySession>, ActivityError> {
        let connection = self.connect()?;
        connection
            .query_row(
                "SELECT id, local_date, started_at_ms, ended_at_ms, active_secs, status, review_status
                 FROM activity_sessions WHERE status='active' ORDER BY started_at_ms DESC LIMIT 1",
                [],
                map_session,
            )
            .optional()
            .map_err(ActivityError::from)
    }

    fn create_session(
        &self,
        local_date: &str,
        now_epoch_ms: i64,
    ) -> Result<ActivitySession, ActivityError> {
        let session = ActivitySession {
            id: Uuid::new_v4().to_string(),
            local_date: local_date.into(),
            started_at_epoch_ms: now_epoch_ms,
            ended_at_epoch_ms: None,
            active_secs: 0.0,
            status: "active".into(),
            review_status: "pending".into(),
        };
        let connection = self.connect()?;
        connection.execute(
            "INSERT INTO activity_sessions(id, local_date, started_at_ms, active_secs, status, review_status)
             VALUES(?1, ?2, ?3, 0, 'active', 'pending')",
            params![session.id, session.local_date, session.started_at_epoch_ms],
        )?;
        Ok(session)
    }

    fn update_session(&self, session: &ActivitySession) -> Result<(), ActivityError> {
        let connection = self.connect()?;
        connection.execute(
            "UPDATE activity_sessions SET ended_at_ms=?2, active_secs=?3, status=?4, review_status=?5 WHERE id=?1",
            params![
                session.id,
                session.ended_at_epoch_ms,
                session.active_secs,
                session.status,
                session.review_status
            ],
        )?;
        Ok(())
    }

    fn resolve_source(
        &self,
        source: &ActivitySourceInput,
    ) -> Result<ResolvedSource, ActivityError> {
        let mut connection = self.connect()?;
        let tx = connection.transaction()?;
        let key = load_or_create_identity_key(&tx)?;
        let normalized_domain = source
            .domain
            .as_deref()
            .map(normalize_domain)
            .filter(|value| !value.is_empty());
        let title_hash = source.title.as_deref().map(|title| keyed_hash(&key, title));
        let identity_key = match source.kind {
            ActivitySourceKind::Browser => format!(
                "browser:{}:{}",
                normalized_domain.as_deref().unwrap_or("unknown"),
                title_hash.as_deref().unwrap_or("untitled")
            ),
            _ => format!("app:{}", source.executable.trim().to_ascii_lowercase()),
        };
        let encrypted_title = source
            .title
            .as_deref()
            .map(|value| protect_data(value.as_bytes()))
            .transpose()?;
        tx.execute(
            "INSERT INTO activity_sources(kind, identity_key, executable, display_name, domain, encrypted_title, title_hash)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(identity_key) DO UPDATE SET display_name=excluded.display_name",
            params![
                source.kind.as_str(),
                identity_key,
                source.executable.to_ascii_lowercase(),
                if source.kind == ActivitySourceKind::Browser {
                    normalized_domain.as_deref().unwrap_or("Google Chrome")
                } else {
                    source.display_name.as_str()
                },
                normalized_domain,
                encrypted_title,
                title_hash
            ],
        )?;
        let id = tx.query_row(
            "SELECT id FROM activity_sources WHERE identity_key=?1",
            params![identity_key],
            |row| row.get(0),
        )?;
        let exact = tx
            .query_row(
                "SELECT category FROM classification_rules WHERE matcher=?1",
                params![identity_key],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        let domain_rule = if source.kind == ActivitySourceKind::Browser {
            normalized_domain
                .as_deref()
                .map(|domain| format!("browser-domain:{domain}"))
        } else {
            None
        };
        let inherited = if exact.is_none() {
            match domain_rule {
                Some(matcher) => tx
                    .query_row(
                        "SELECT category FROM classification_rules WHERE matcher=?1",
                        params![matcher],
                        |row| row.get::<_, String>(0),
                    )
                    .optional()?,
                None => None,
            }
        } else {
            None
        };
        tx.commit()?;
        Ok(ResolvedSource {
            id,
            identity_key,
            classification: exact
                .or(inherited)
                .and_then(|value| ActivityCategory::parse(&value)),
        })
    }

    fn insert_segment(&self, segment: &OpenSegment) -> Result<(), ActivityError> {
        if segment.duration_secs <= 0.0 {
            return Ok(());
        }
        let Some(session_id) = segment.session_id.as_deref() else {
            return Ok(());
        };
        let connection = self.connect()?;
        connection.execute(
            "INSERT INTO activity_segments(
                session_id, source_id, started_at_ms, ended_at_ms, duration_secs, credited_secs, bucket, classification
             ) VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                session_id,
                segment.source_id,
                segment.started_at_epoch_ms,
                segment.ended_at_epoch_ms,
                segment.duration_secs,
                segment.credited_secs,
                segment.bucket,
                segment.classification.as_ref().map(ActivityCategory::as_str)
            ],
        )?;
        Ok(())
    }

    pub fn pending_reviews(&self) -> Result<u64, ActivityError> {
        let connection = self.connect()?;
        let count = connection.query_row(
            "SELECT COUNT(*) FROM activity_sessions WHERE status IN ('complete', 'partial') AND review_status='pending'",
            [],
            |row| row.get::<_, i64>(0),
        )?;
        Ok(count.max(0) as u64)
    }

    pub fn session_review(&self, session_id: &str) -> Result<SessionReview, ActivityError> {
        let connection = self.connect()?;
        let session = connection
            .query_row(
                "SELECT id, local_date, started_at_ms, ended_at_ms, active_secs, status, review_status
                 FROM activity_sessions WHERE id=?1",
                params![session_id],
                map_session,
            )
            .optional()?
            .ok_or_else(|| ActivityError::Data("Activity session not found".into()))?;
        build_review(&connection, session)
    }

    pub fn current_session_review(&self) -> Result<Option<SessionReview>, ActivityError> {
        let Some(session) = self.active_session()? else {
            return Ok(None);
        };
        let connection = self.connect()?;
        Ok(Some(build_review(&connection, session)?))
    }

    pub fn dashboard(&self, days: u32) -> Result<ActivityDashboard, ActivityError> {
        let days = days.clamp(1, 90);
        let first_date = (Local::now().date_naive() - ChronoDuration::days(days as i64 - 1))
            .format("%Y-%m-%d")
            .to_string();
        let connection = self.connect()?;
        let mut statement = connection.prepare(
            "SELECT id, local_date, started_at_ms, ended_at_ms, active_secs, status, review_status
             FROM activity_sessions WHERE local_date>=?1 ORDER BY started_at_ms DESC",
        )?;
        let sessions = statement
            .query_map(params![first_date], map_session)?
            .collect::<Result<Vec<_>, _>>()?;
        drop(statement);

        let mut category_totals = BTreeMap::<String, f64>::new();
        let mut aggregate_map = HashMap::<i64, ActivityAggregate>::new();
        let mut daily_map = BTreeMap::<String, DailyActivitySummary>::new();
        let mut active_secs = 0.0;
        let mut break_secs = 0.0;
        for session in &sessions {
            active_secs += session.active_secs;
            let review = build_review(&connection, session.clone())?;
            for (category, seconds) in review.category_totals {
                *category_totals.entry(category.clone()).or_default() += seconds;
                *daily_map
                    .entry(session.local_date.clone())
                    .or_insert_with(|| DailyActivitySummary {
                        local_date: session.local_date.clone(),
                        category_totals: BTreeMap::new(),
                        completed_sessions: 0,
                        partial_sessions: 0,
                    })
                    .category_totals
                    .entry(category.clone())
                    .or_default() += seconds;
                if category == "break" {
                    break_secs += seconds;
                }
            }
            let daily = daily_map
                .entry(session.local_date.clone())
                .or_insert_with(|| DailyActivitySummary {
                    local_date: session.local_date.clone(),
                    category_totals: BTreeMap::new(),
                    completed_sessions: 0,
                    partial_sessions: 0,
                });
            if session.status == "complete" {
                daily.completed_sessions += 1;
            } else if session.status == "partial" {
                daily.partial_sessions += 1;
            }
            for activity in review.activities {
                aggregate_map
                    .entry(activity.source.id)
                    .and_modify(|current| current.duration_secs += activity.duration_secs)
                    .or_insert(activity);
            }
        }
        let mut activities = aggregate_map.into_values().collect::<Vec<_>>();
        activities.sort_by(|left, right| right.duration_secs.total_cmp(&left.duration_secs));
        let mut daily = daily_map.into_values().collect::<Vec<_>>();
        daily.sort_by(|left, right| left.local_date.cmp(&right.local_date));

        Ok(ActivityDashboard {
            range_days: days,
            sessions,
            activities,
            category_totals,
            daily,
            pending_reviews: self.pending_reviews()?,
            active_secs,
            break_secs,
        })
    }

    pub fn classify_activity(
        &self,
        session_id: &str,
        source_id: i64,
        category: ActivityCategory,
        use_next_time: bool,
        domain_wide: bool,
        now_epoch_ms: i64,
    ) -> Result<(), ActivityError> {
        let mut connection = self.connect()?;
        let tx = connection.transaction()?;
        tx.execute(
            "UPDATE activity_segments SET classification=?3 WHERE session_id=?1 AND source_id=?2",
            params![session_id, source_id, category.as_str()],
        )?;
        if use_next_time {
            save_rule_for_source(&tx, source_id, &category, domain_wide, now_epoch_ms)?;
        }
        update_review_status(&tx, session_id)?;
        tx.commit()?;
        Ok(())
    }

    pub fn complete_session_review(
        &self,
        session_id: &str,
        classifications: &[SessionClassification],
        now_epoch_ms: i64,
    ) -> Result<(), ActivityError> {
        let mut connection = self.connect()?;
        let tx = connection.transaction()?;
        let status = tx
            .query_row(
                "SELECT status FROM activity_sessions WHERE id=?1",
                params![session_id],
                |row| row.get::<_, String>(0),
            )
            .optional()?
            .ok_or_else(|| ActivityError::Data("Activity session not found".into()))?;
        if status == "active" {
            return Err(ActivityError::Data(
                "The current session can be reviewed after it is complete".into(),
            ));
        }

        for classification in classifications {
            let changed = tx.execute(
                "UPDATE activity_segments SET classification=?3
                 WHERE session_id=?1 AND source_id=?2",
                params![
                    session_id,
                    classification.source_id,
                    classification.category.as_str()
                ],
            )?;
            if changed == 0 {
                return Err(ActivityError::Data(
                    "An activity no longer belongs to this session".into(),
                ));
            }
            if classification.use_next_time {
                save_rule_for_source(
                    &tx,
                    classification.source_id,
                    &classification.category,
                    classification.domain_wide,
                    now_epoch_ms,
                )?;
            }
        }

        let pending = qualifying_unclassified_count(&tx, session_id)?;
        if pending > 0 {
            return Err(ActivityError::Data(
                "Choose a category for every meaningful activity".into(),
            ));
        }
        tx.execute(
            "UPDATE activity_sessions SET review_status='reviewed' WHERE id=?1",
            params![session_id],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn save_rule(
        &self,
        source_id: i64,
        category: ActivityCategory,
        domain_wide: bool,
        now_epoch_ms: i64,
    ) -> Result<(), ActivityError> {
        let mut connection = self.connect()?;
        let tx = connection.transaction()?;
        save_rule_for_source(&tx, source_id, &category, domain_wide, now_epoch_ms)?;
        tx.commit()?;
        Ok(())
    }

    pub fn rules(&self) -> Result<Vec<ClassificationRule>, ActivityError> {
        let connection = self.connect()?;
        let mut statement = connection.prepare(
            "SELECT id, source_kind, matcher, display_name, category, domain_wide, created_at_ms
             FROM classification_rules ORDER BY created_at_ms DESC",
        )?;
        let values = statement
            .query_map([], |row| {
                let category: String = row.get(4)?;
                Ok(ClassificationRule {
                    id: row.get(0)?,
                    source_kind: ActivitySourceKind::parse(&row.get::<_, String>(1)?),
                    matcher: row.get(2)?,
                    display_name: row.get(3)?,
                    category: ActivityCategory::parse(&category)
                        .unwrap_or(ActivityCategory::Neutral),
                    domain_wide: row.get::<_, i64>(5)? != 0,
                    created_at_epoch_ms: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(values)
    }

    pub fn delete_rule(&self, id: i64) -> Result<(), ActivityError> {
        self.connect()?
            .execute("DELETE FROM classification_rules WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn prune(&self, retention_days: u32) -> Result<(), ActivityError> {
        let cutoff = (Local::now().date_naive() - ChronoDuration::days(retention_days as i64))
            .format("%Y-%m-%d")
            .to_string();
        let connection = self.connect()?;
        connection.execute(
            "DELETE FROM activity_sessions WHERE local_date<?1 AND status!='active'",
            params![cutoff],
        )?;
        connection.execute(
            "DELETE FROM activity_sources WHERE id NOT IN (SELECT DISTINCT source_id FROM activity_segments WHERE source_id IS NOT NULL)",
            [],
        )?;
        Ok(())
    }

    pub fn export_json(&self, output: &Path) -> Result<(), ActivityError> {
        let dashboard = self.dashboard(90)?;
        let data = serde_json::to_vec_pretty(&dashboard)
            .map_err(|error| ActivityError::Data(error.to_string()))?;
        fs::write(output, data)?;
        Ok(())
    }

    pub fn export_csv(&self, output: &Path) -> Result<(), ActivityError> {
        let dashboard = self.dashboard(90)?;
        let mut csv = String::from("date,session_id,session_status,source_type,source,domain,title,duration_seconds,category\n");
        for session in dashboard.sessions {
            let review = self.session_review(&session.id)?;
            for activity in review.activities {
                csv.push_str(&format!(
                    "{},{},{},{},{},{},{},{:.3},{}\n",
                    csv_escape(&session.local_date),
                    csv_escape(&session.id),
                    csv_escape(&session.status),
                    csv_escape(activity.source.kind.as_str()),
                    csv_escape(&activity.source.name),
                    csv_escape(activity.source.domain.as_deref().unwrap_or_default()),
                    csv_escape(activity.source.title.as_deref().unwrap_or_default()),
                    activity.duration_secs,
                    csv_escape(
                        activity
                            .category
                            .as_ref()
                            .map(ActivityCategory::as_str)
                            .unwrap_or("unclassified")
                    )
                ));
            }
            if review.short_activity_secs > 0.0 {
                csv.push_str(&format!(
                    "{},{},{},system,Short activity,,,{:.3},shortActivity\n",
                    csv_escape(&session.local_date),
                    csv_escape(&session.id),
                    csv_escape(&session.status),
                    review.short_activity_secs
                ));
            }
        }
        fs::write(output, csv)?;
        Ok(())
    }

    pub fn delete_history(&self) -> Result<(), ActivityError> {
        let connection = self.connect()?;
        connection.execute_batch(
            "DELETE FROM activity_segments;
             DELETE FROM activity_sessions;
             DELETE FROM activity_sources;
             DELETE FROM classification_rules;
             DELETE FROM activity_metadata WHERE key!='tracking_paused';
             PRAGMA wal_checkpoint(TRUNCATE);
             VACUUM;",
        )?;
        drop(connection);
        for suffix in ["-wal", "-shm"] {
            let sidecar = PathBuf::from(format!("{}{}", self.path.display(), suffix));
            if sidecar.exists() {
                fs::remove_file(sidecar)?;
            }
        }
        if let Some(parent) = self.path.parent() {
            let stem = self
                .path
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("activity");
            for entry in fs::read_dir(parent)? {
                let path = entry?.path();
                let name = path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or_default();
                if name.starts_with(&format!("{stem}.pre-v"))
                    && path.extension().and_then(|value| value.to_str()) == Some("db")
                {
                    fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct OpenSegment {
    key: String,
    session_id: Option<String>,
    source_id: Option<i64>,
    started_at_epoch_ms: i64,
    ended_at_epoch_ms: i64,
    duration_secs: f64,
    credited_secs: f64,
    bucket: String,
    classification: Option<ActivityCategory>,
}

#[derive(Debug)]
pub struct ActivityEngine {
    repository: ActivityRepository,
    settings: ActivitySettings,
    paused: bool,
    current_session: Option<ActivitySession>,
    open: Option<OpenSegment>,
    seconds_since_flush: f64,
    source_cache: HashMap<String, ResolvedSource>,
}

impl ActivityEngine {
    pub fn new(
        repository: ActivityRepository,
        settings: ActivitySettings,
        now_epoch_ms: i64,
        local_date: &str,
    ) -> Result<Self, ActivityError> {
        let paused = repository.tracking_paused()?;
        let mut current_session = repository.active_session()?;
        if let Some(session) = current_session.as_mut() {
            if session.local_date != local_date {
                session.status = "partial".into();
                session.ended_at_epoch_ms = Some(now_epoch_ms);
                repository.update_session(session)?;
                current_session = None;
            }
        }
        Ok(Self {
            repository,
            settings,
            paused,
            current_session,
            open: None,
            seconds_since_flush: 0.0,
            source_cache: HashMap::new(),
        })
    }

    pub fn repository(&self) -> &ActivityRepository {
        &self.repository
    }

    pub fn update_settings(&mut self, settings: ActivitySettings) {
        self.settings = settings;
    }

    pub fn clear_source_cache(&mut self) {
        self.source_cache.clear();
    }

    pub fn delete_history(&mut self, now_epoch_ms: i64) -> Result<(), ActivityError> {
        self.flush(now_epoch_ms)?;
        self.repository.delete_history()?;
        self.current_session = None;
        self.open = None;
        self.source_cache.clear();
        Ok(())
    }

    pub fn pause(&mut self, now_epoch_ms: i64) -> Result<(), ActivityError> {
        self.flush(now_epoch_ms)?;
        self.paused = true;
        self.repository.set_tracking_paused(true)
    }

    pub fn resume(&mut self, now_epoch_ms: i64) -> Result<(), ActivityError> {
        self.flush(now_epoch_ms)?;
        self.paused = false;
        self.repository.set_tracking_paused(false)
    }

    pub fn status(&self) -> Result<TrackingStatus, ActivityError> {
        let status = if !self.settings.consented {
            "Consent required"
        } else if !self.settings.enabled {
            "Tracking off"
        } else if self.paused {
            "Tracking paused"
        } else {
            "Tracking activity"
        };
        Ok(TrackingStatus {
            consented: self.settings.consented,
            enabled: self.settings.enabled,
            paused: self.paused,
            status: status.into(),
            current_session_id: self.current_session.as_ref().map(|value| value.id.clone()),
            current_session_active_secs: self
                .current_session
                .as_ref()
                .map(|value| value.active_secs)
                .unwrap_or_default(),
            session_target_secs: SESSION_TARGET_SECS,
            pending_reviews: self.repository.pending_reviews()?,
        })
    }

    pub fn current_session(&self) -> Option<ActivitySession> {
        self.current_session.clone()
    }

    pub fn tick(&mut self, tick: ActivityTick) -> Result<Vec<ActivityEvent>, ActivityError> {
        let mut events = Vec::new();
        let delta = if tick.delta_secs > 5.0 {
            0.0
        } else {
            tick.delta_secs.clamp(0.0, 2.0)
        };
        if delta <= 0.0 {
            return Ok(events);
        }

        if self
            .current_session
            .as_ref()
            .is_some_and(|session| session.local_date != tick.local_date)
        {
            self.flush(tick.now_epoch_ms)?;
            if let Some(mut session) = self.current_session.take() {
                session.status = "partial".into();
                session.ended_at_epoch_ms = Some(tick.now_epoch_ms);
                self.repository.update_session(&session)?;
                events.push(ActivityEvent::SessionChanged);
            }
        }

        let desired = self.desired_segment(&tick)?;
        let credited = desired.as_ref().is_some_and(|value| value.credited);
        let available = self
            .current_session
            .as_ref()
            .map(|session| (SESSION_TARGET_SECS - session.active_secs).max(0.0))
            .unwrap_or(SESSION_TARGET_SECS);

        if credited && delta > available && available > 0.0 {
            self.apply_slice(
                desired.clone(),
                available,
                tick.now_epoch_ms - ((delta - available) * 1000.0) as i64,
                &tick.local_date,
            )?;
            self.flush(tick.now_epoch_ms)?;
            if let Some(mut completed) = self.current_session.take() {
                completed.active_secs = SESSION_TARGET_SECS;
                completed.status = "complete".into();
                completed.ended_at_epoch_ms = Some(tick.now_epoch_ms);
                self.repository.update_session(&completed)?;
                events.push(ActivityEvent::SessionCompleted(completed));
            }
            self.apply_slice(
                desired,
                delta - available,
                tick.now_epoch_ms,
                &tick.local_date,
            )?;
        } else {
            self.apply_slice(desired, delta, tick.now_epoch_ms, &tick.local_date)?;
            if credited
                && self
                    .current_session
                    .as_ref()
                    .is_some_and(|session| session.active_secs >= SESSION_TARGET_SECS)
            {
                self.flush(tick.now_epoch_ms)?;
                if let Some(mut completed) = self.current_session.take() {
                    completed.active_secs = SESSION_TARGET_SECS;
                    completed.status = "complete".into();
                    completed.ended_at_epoch_ms = Some(tick.now_epoch_ms);
                    self.repository.update_session(&completed)?;
                    events.push(ActivityEvent::SessionCompleted(completed));
                }
            }
        }

        self.seconds_since_flush += delta;
        if self.seconds_since_flush >= 15.0 {
            self.flush(tick.now_epoch_ms)?;
            events.push(ActivityEvent::SessionChanged);
        }
        Ok(events)
    }

    fn desired_segment(
        &mut self,
        tick: &ActivityTick,
    ) -> Result<Option<DesiredSegment>, ActivityError> {
        if !self.settings.consented || !self.settings.enabled {
            return Ok(None);
        }
        if self.paused {
            return Ok(Some(DesiredSegment::system(
                "tracking-paused",
                "Tracking paused",
                false,
            )));
        }
        if tick.break_active {
            return Ok(Some(DesiredSegment {
                key: "system:break".into(),
                source_id: None,
                bucket: "break".into(),
                classification: Some(ActivityCategory::Break),
                credited: false,
            }));
        }
        if tick.locked_or_sleeping {
            return Ok(Some(DesiredSegment::system("away", "Away", false)));
        }

        let Some(source) = tick.source.as_ref() else {
            return Ok(Some(DesiredSegment::system("away", "Away", false)));
        };
        if tick.idle_secs >= self.settings.idle_cutoff_secs {
            return Ok(Some(DesiredSegment::system("away", "Away", false)));
        }

        let excluded_app = self.settings.excluded_apps.iter().any(|value| {
            value.eq_ignore_ascii_case(&source.executable)
                || value.eq_ignore_ascii_case(&source.display_name)
        });
        if excluded_app {
            return Ok(Some(DesiredSegment {
                key: "system:private".into(),
                source_id: None,
                bucket: "private".into(),
                classification: None,
                credited: true,
            }));
        }

        let memory_key = source.memory_key();
        let resolved = if let Some(cached) = self.source_cache.get(&memory_key) {
            cached.clone()
        } else {
            let resolved = self.repository.resolve_source(source)?;
            self.source_cache
                .insert(memory_key.clone(), resolved.clone());
            resolved
        };
        Ok(Some(DesiredSegment {
            key: format!("{}|{}", memory_key, resolved.identity_key),
            source_id: Some(resolved.id),
            bucket: "activity".into(),
            classification: resolved.classification,
            credited: true,
        }))
    }

    fn apply_slice(
        &mut self,
        desired: Option<DesiredSegment>,
        delta: f64,
        now_epoch_ms: i64,
        local_date: &str,
    ) -> Result<(), ActivityError> {
        if delta <= 0.0 {
            return Ok(());
        }
        let Some(desired) = desired else {
            self.flush(now_epoch_ms)?;
            return Ok(());
        };
        if desired.credited && self.current_session.is_none() {
            self.current_session = Some(self.repository.create_session(local_date, now_epoch_ms)?);
        }
        let session_id = self.current_session.as_ref().map(|value| value.id.clone());
        let changed = self
            .open
            .as_ref()
            .is_some_and(|open| open.key != desired.key || open.session_id != session_id);
        if changed {
            self.flush(now_epoch_ms)?;
        }
        if self.open.is_none() {
            self.open = Some(OpenSegment {
                key: desired.key,
                session_id,
                source_id: desired.source_id,
                started_at_epoch_ms: now_epoch_ms - (delta * 1000.0) as i64,
                ended_at_epoch_ms: now_epoch_ms,
                duration_secs: 0.0,
                credited_secs: 0.0,
                bucket: desired.bucket,
                classification: desired.classification,
            });
        }
        if let Some(open) = self.open.as_mut() {
            open.ended_at_epoch_ms = now_epoch_ms;
            open.duration_secs += delta;
            if desired.credited {
                open.credited_secs += delta;
            }
        }
        if desired.credited {
            if let Some(session) = self.current_session.as_mut() {
                session.active_secs = (session.active_secs + delta).min(SESSION_TARGET_SECS);
            }
        }
        Ok(())
    }

    pub fn flush(&mut self, now_epoch_ms: i64) -> Result<(), ActivityError> {
        if let Some(mut open) = self.open.take() {
            open.ended_at_epoch_ms = now_epoch_ms.max(open.started_at_epoch_ms);
            self.repository.insert_segment(&open)?;
        }
        if let Some(session) = self.current_session.as_ref() {
            self.repository.update_session(session)?;
        }
        self.seconds_since_flush = 0.0;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct DesiredSegment {
    key: String,
    source_id: Option<i64>,
    bucket: String,
    classification: Option<ActivityCategory>,
    credited: bool,
}

impl DesiredSegment {
    fn system(key: &str, bucket: &str, credited: bool) -> Self {
        Self {
            key: format!("system:{key}"),
            source_id: None,
            bucket: bucket.to_ascii_lowercase().replace(' ', "-"),
            classification: None,
            credited,
        }
    }
}

fn map_session(row: &rusqlite::Row<'_>) -> rusqlite::Result<ActivitySession> {
    Ok(ActivitySession {
        id: row.get(0)?,
        local_date: row.get(1)?,
        started_at_epoch_ms: row.get(2)?,
        ended_at_epoch_ms: row.get(3)?,
        active_secs: row.get(4)?,
        status: row.get(5)?,
        review_status: row.get(6)?,
    })
}

fn build_review(
    connection: &Connection,
    session: ActivitySession,
) -> Result<SessionReview, ActivityError> {
    let mut statement = connection.prepare(
        "SELECT s.id, s.kind, s.executable, s.display_name, s.domain, s.encrypted_title,
                SUM(g.duration_secs), MAX(g.classification)
         FROM activity_segments g
         JOIN activity_sources s ON s.id=g.source_id
         WHERE g.session_id=?1
         GROUP BY s.id, s.kind, s.executable, s.display_name, s.domain, s.encrypted_title
         ORDER BY SUM(g.duration_secs) DESC",
    )?;
    let rows = statement.query_map(params![session.id], |row| {
        let encrypted: Option<Vec<u8>> = row.get(5)?;
        let category: Option<String> = row.get(7)?;
        let title = encrypted
            .as_deref()
            .and_then(|value| unprotect_data(value).ok())
            .and_then(|value| String::from_utf8(value).ok());
        Ok(ActivityAggregate {
            source: ActivitySource {
                id: row.get(0)?,
                kind: ActivitySourceKind::parse(&row.get::<_, String>(1)?),
                executable: row.get(2)?,
                name: row.get(3)?,
                domain: row.get(4)?,
                title,
            },
            duration_secs: row.get(6)?,
            category: category.as_deref().and_then(ActivityCategory::parse),
            qualifying: false,
        })
    })?;
    let all_activities = rows.collect::<Result<Vec<_>, _>>()?;
    drop(statement);

    let mut activities = Vec::new();
    let mut category_totals = BTreeMap::<String, f64>::new();
    let mut short_activity_secs = 0.0;
    let mut pending_count = 0;
    for mut activity in all_activities {
        activity.qualifying = activity.duration_secs >= REVIEW_THRESHOLD_SECS;
        if activity.qualifying {
            let bucket = activity
                .category
                .as_ref()
                .map(ActivityCategory::as_str)
                .unwrap_or("unclassified");
            *category_totals.entry(bucket.into()).or_default() += activity.duration_secs;
            if activity.category.is_none() {
                pending_count += 1;
            }
            activities.push(activity);
        } else {
            short_activity_secs += activity.duration_secs;
        }
    }
    if short_activity_secs > 0.0 {
        category_totals.insert("shortActivity".into(), short_activity_secs);
    }

    let mut bucket_statement = connection.prepare(
        "SELECT bucket, SUM(duration_secs) FROM activity_segments
         WHERE session_id=?1 AND source_id IS NULL GROUP BY bucket",
    )?;
    let buckets = bucket_statement
        .query_map(params![session.id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    for (bucket, seconds) in buckets {
        let normalized = match bucket.as_str() {
            "private" => "privateActivity",
            "break" => "break",
            "away" => "away",
            value => value,
        };
        *category_totals.entry(normalized.into()).or_default() += seconds;
    }

    let mut timeline_statement = connection.prepare(
        "SELECT id, session_id, source_id, started_at_ms, ended_at_ms, duration_secs,
                credited_secs, bucket, classification
         FROM activity_segments WHERE session_id=?1 ORDER BY started_at_ms",
    )?;
    let timeline = timeline_statement
        .query_map(params![session.id], |row| {
            let category: Option<String> = row.get(8)?;
            Ok(ActivitySegment {
                id: row.get(0)?,
                session_id: row.get(1)?,
                source_id: row.get(2)?,
                started_at_epoch_ms: row.get(3)?,
                ended_at_epoch_ms: row.get(4)?,
                duration_secs: row.get(5)?,
                credited_secs: row.get(6)?,
                bucket: row.get(7)?,
                category: category.as_deref().and_then(ActivityCategory::parse),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    let short_switch_count = timeline
        .iter()
        .filter(|segment| {
            segment.source_id.is_some() && segment.duration_secs < REVIEW_THRESHOLD_SECS
        })
        .count() as u64;

    Ok(SessionReview {
        session,
        activities,
        timeline,
        category_totals,
        short_activity_secs,
        short_switch_count,
        pending_count,
    })
}

fn save_rule_for_source(
    tx: &Transaction<'_>,
    source_id: i64,
    category: &ActivityCategory,
    domain_wide: bool,
    now_epoch_ms: i64,
) -> Result<(), ActivityError> {
    let (kind, identity_key, display_name, domain): (String, String, String, Option<String>) = tx
        .query_row(
        "SELECT kind, identity_key, display_name, domain FROM activity_sources WHERE id=?1",
        params![source_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    )?;
    let matcher = if kind == "browser" && domain_wide {
        format!("browser-domain:{}", domain.as_deref().unwrap_or("unknown"))
    } else {
        identity_key
    };
    tx.execute(
        "INSERT INTO classification_rules(source_kind, matcher, display_name, category, domain_wide, created_at_ms)
         VALUES(?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(matcher) DO UPDATE SET category=excluded.category, domain_wide=excluded.domain_wide",
        params![kind, matcher, display_name, category.as_str(), domain_wide as i64, now_epoch_ms],
    )?;
    Ok(())
}

fn update_review_status(tx: &Transaction<'_>, session_id: &str) -> Result<(), ActivityError> {
    let pending = qualifying_unclassified_count(tx, session_id)?;
    tx.execute(
        "UPDATE activity_sessions SET review_status=?2 WHERE id=?1",
        params![
            session_id,
            if pending == 0 { "reviewed" } else { "pending" }
        ],
    )?;
    Ok(())
}

fn qualifying_unclassified_count(
    tx: &Transaction<'_>,
    session_id: &str,
) -> Result<i64, ActivityError> {
    Ok(tx.query_row(
        "SELECT COUNT(*) FROM (
            SELECT source_id FROM activity_segments WHERE session_id=?1 AND source_id IS NOT NULL
            GROUP BY source_id HAVING SUM(duration_secs)>=?2 AND MAX(classification) IS NULL
         )",
        params![session_id, REVIEW_THRESHOLD_SECS],
        |row| row.get::<_, i64>(0),
    )?)
}

fn load_or_create_identity_key(tx: &Transaction<'_>) -> Result<Vec<u8>, ActivityError> {
    if let Some(encrypted) = tx
        .query_row(
            "SELECT value FROM activity_metadata WHERE key='identity_key'",
            [],
            |row| row.get::<_, Vec<u8>>(0),
        )
        .optional()?
    {
        return unprotect_data(&encrypted);
    }
    let mut key = Vec::with_capacity(32);
    key.extend_from_slice(Uuid::new_v4().as_bytes());
    key.extend_from_slice(Uuid::new_v4().as_bytes());
    let encrypted = protect_data(&key)?;
    tx.execute(
        "INSERT INTO activity_metadata(key, value) VALUES('identity_key', ?1)",
        params![encrypted],
    )?;
    Ok(key)
}

fn keyed_hash(key: &[u8], value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key);
    hasher.update(value.trim().to_lowercase().as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn normalize_domain(value: &str) -> String {
    value
        .trim()
        .trim_end_matches('.')
        .trim_start_matches("www.")
        .to_ascii_lowercase()
}

fn csv_escape(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.into()
    }
}

#[cfg(windows)]
fn protect_data(value: &[u8]) -> Result<Vec<u8>, ActivityError> {
    use std::ptr;
    use windows_sys::Win32::{
        Foundation::LocalFree,
        Security::Cryptography::{CryptProtectData, CRYPTPROTECT_UI_FORBIDDEN, CRYPT_INTEGER_BLOB},
    };
    let input = CRYPT_INTEGER_BLOB {
        cbData: value.len() as u32,
        pbData: value.as_ptr() as *mut u8,
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: ptr::null_mut(),
    };
    let success = unsafe {
        CryptProtectData(
            &input,
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if success == 0 {
        return Err(ActivityError::Data(format!(
            "DPAPI encryption failed: {}",
            std::io::Error::last_os_error()
        )));
    }
    let bytes =
        unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize) }.to_vec();
    unsafe { LocalFree(output.pbData as *mut core::ffi::c_void) };
    Ok(bytes)
}

#[cfg(windows)]
fn unprotect_data(value: &[u8]) -> Result<Vec<u8>, ActivityError> {
    use std::ptr;
    use windows_sys::Win32::{
        Foundation::LocalFree,
        Security::Cryptography::{
            CryptUnprotectData, CRYPTPROTECT_UI_FORBIDDEN, CRYPT_INTEGER_BLOB,
        },
    };
    let input = CRYPT_INTEGER_BLOB {
        cbData: value.len() as u32,
        pbData: value.as_ptr() as *mut u8,
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: ptr::null_mut(),
    };
    let success = unsafe {
        CryptUnprotectData(
            &input,
            ptr::null_mut(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if success == 0 {
        return Err(ActivityError::Data(format!(
            "DPAPI decryption failed: {}",
            std::io::Error::last_os_error()
        )));
    }
    let bytes =
        unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize) }.to_vec();
    unsafe { LocalFree(output.pbData as *mut core::ffi::c_void) };
    Ok(bytes)
}

#[cfg(not(windows))]
fn protect_data(value: &[u8]) -> Result<Vec<u8>, ActivityError> {
    Ok(value.to_vec())
}

#[cfg(not(windows))]
fn unprotect_data(value: &[u8]) -> Result<Vec<u8>, ActivityError> {
    Ok(value.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_repository(name: &str) -> ActivityRepository {
        let path = std::env::temp_dir().join(format!("peeky-{name}-{}.db", Uuid::new_v4()));
        ActivityRepository::initialize(path).unwrap()
    }

    fn settings() -> ActivitySettings {
        ActivitySettings {
            consented: true,
            enabled: true,
            ..ActivitySettings::default()
        }
    }

    fn app_source() -> ActivitySourceInput {
        ActivitySourceInput {
            kind: ActivitySourceKind::Application,
            executable: "code.exe".into(),
            display_name: "Visual Studio Code".into(),
            domain: None,
            title: None,
            audible: false,
        }
    }

    #[test]
    fn exact_two_hour_session_completes() {
        let repository = test_repository("two-hour");
        let mut engine = ActivityEngine::new(repository, settings(), 0, "2026-08-12").unwrap();
        let mut completed = 0;
        for second in 1..=7_200 {
            let events = engine
                .tick(ActivityTick {
                    now_epoch_ms: second * 1_000,
                    local_date: "2026-08-12".into(),
                    delta_secs: 1.0,
                    idle_secs: 0,
                    locked_or_sleeping: false,
                    break_active: false,
                    source: Some(app_source()),
                })
                .unwrap();
            completed += events
                .iter()
                .filter(|event| matches!(event, ActivityEvent::SessionCompleted(_)))
                .count();
        }
        assert_eq!(completed, 1);
        assert!(engine.current_session().is_none());
    }

    #[test]
    fn idle_and_break_time_do_not_advance_session() {
        let repository = test_repository("idle-break");
        let mut engine = ActivityEngine::new(repository, settings(), 0, "2026-08-12").unwrap();
        for second in 1..=20 {
            engine
                .tick(ActivityTick {
                    now_epoch_ms: second * 1_000,
                    local_date: "2026-08-12".into(),
                    delta_secs: 1.0,
                    idle_secs: if second > 10 { 400 } else { 0 },
                    locked_or_sleeping: false,
                    break_active: second > 5 && second <= 10,
                    source: Some(app_source()),
                })
                .unwrap();
        }
        assert_eq!(engine.current_session().unwrap().active_secs, 5.0);
    }

    #[test]
    fn chrome_is_stored_as_one_application_identity() {
        let repository = test_repository("chrome-app");
        let chrome = ActivitySourceInput {
            kind: ActivitySourceKind::Application,
            executable: "chrome.exe".into(),
            display_name: "Google Chrome".into(),
            domain: None,
            title: None,
            audible: false,
        };
        let mut engine =
            ActivityEngine::new(repository.clone(), settings(), 0, "2026-08-12").unwrap();
        for second in 1..=180 {
            engine
                .tick(ActivityTick {
                    now_epoch_ms: second * 1_000,
                    local_date: "2026-08-12".into(),
                    delta_secs: 1.0,
                    idle_secs: 0,
                    locked_or_sleeping: false,
                    break_active: false,
                    source: Some(chrome.clone()),
                })
                .unwrap();
        }
        engine.flush(180_000).unwrap();
        let review = repository.current_session_review().unwrap().unwrap();
        assert_eq!(review.activities.len(), 1);
        assert_eq!(review.activities[0].source.name, "Google Chrome");
        assert_eq!(
            review.activities[0].source.kind,
            ActivitySourceKind::Application
        );
        assert!(review.activities[0].source.domain.is_none());
        assert!(review.activities[0].source.title.is_none());
    }

    #[test]
    fn midnight_closes_the_previous_session_as_partial() {
        let repository = test_repository("midnight");
        let mut engine =
            ActivityEngine::new(repository.clone(), settings(), 0, "2026-08-12").unwrap();
        engine
            .tick(ActivityTick {
                now_epoch_ms: 1_000,
                local_date: "2026-08-12".into(),
                delta_secs: 1.0,
                idle_secs: 0,
                locked_or_sleeping: false,
                break_active: false,
                source: Some(app_source()),
            })
            .unwrap();
        engine
            .tick(ActivityTick {
                now_epoch_ms: 2_000,
                local_date: "2026-08-13".into(),
                delta_secs: 1.0,
                idle_secs: 0,
                locked_or_sleeping: false,
                break_active: false,
                source: Some(app_source()),
            })
            .unwrap();
        let dashboard = repository.dashboard(90).unwrap();
        assert!(dashboard
            .sessions
            .iter()
            .any(|session| session.status == "partial"));
        assert_eq!(engine.current_session().unwrap().local_date, "2026-08-13");
    }

    #[test]
    fn short_partial_session_can_be_explicitly_reviewed() {
        let repository = test_repository("review-short-partial");
        let mut engine =
            ActivityEngine::new(repository.clone(), settings(), 0, "2026-08-12").unwrap();
        engine
            .tick(ActivityTick {
                now_epoch_ms: 1_000,
                local_date: "2026-08-12".into(),
                delta_secs: 1.0,
                idle_secs: 0,
                locked_or_sleeping: false,
                break_active: false,
                source: Some(app_source()),
            })
            .unwrap();
        engine
            .tick(ActivityTick {
                now_epoch_ms: 2_000,
                local_date: "2026-08-13".into(),
                delta_secs: 1.0,
                idle_secs: 0,
                locked_or_sleeping: false,
                break_active: false,
                source: Some(app_source()),
            })
            .unwrap();

        let partial = repository
            .dashboard(90)
            .unwrap()
            .sessions
            .into_iter()
            .find(|session| session.local_date == "2026-08-12")
            .unwrap();
        repository
            .complete_session_review(&partial.id, &[], 3_000)
            .unwrap();

        assert_eq!(
            repository.session_review(&partial.id).unwrap().session.review_status,
            "reviewed"
        );
        assert_eq!(repository.pending_reviews().unwrap(), 0);
    }

    #[test]
    fn meaningful_activity_requires_context_before_session_review() {
        let repository = test_repository("review-meaningful");
        let mut engine =
            ActivityEngine::new(repository.clone(), settings(), 0, "2026-08-12").unwrap();
        for second in 1..=180 {
            engine
                .tick(ActivityTick {
                    now_epoch_ms: second * 1_000,
                    local_date: "2026-08-12".into(),
                    delta_secs: 1.0,
                    idle_secs: 0,
                    locked_or_sleeping: false,
                    break_active: false,
                    source: Some(app_source()),
                })
                .unwrap();
        }
        engine
            .tick(ActivityTick {
                now_epoch_ms: 181_000,
                local_date: "2026-08-13".into(),
                delta_secs: 1.0,
                idle_secs: 0,
                locked_or_sleeping: false,
                break_active: false,
                source: Some(app_source()),
            })
            .unwrap();

        let partial = repository
            .dashboard(90)
            .unwrap()
            .sessions
            .into_iter()
            .find(|session| session.local_date == "2026-08-12")
            .unwrap();
        let review = repository.session_review(&partial.id).unwrap();
        assert!(repository
            .complete_session_review(&partial.id, &[], 182_000)
            .is_err());

        repository
            .complete_session_review(
                &partial.id,
                &[SessionClassification {
                    source_id: review.activities[0].source.id,
                    category: ActivityCategory::Productive,
                    use_next_time: false,
                    domain_wide: false,
                }],
                182_000,
            )
            .unwrap();
        let reviewed = repository.session_review(&partial.id).unwrap();
        assert_eq!(reviewed.session.review_status, "reviewed");
        assert_eq!(reviewed.activities[0].category, Some(ActivityCategory::Productive));
    }

    #[test]
    fn chrome_stops_at_the_same_idle_cutoff_as_other_apps() {
        let repository = test_repository("chrome-idle");
        let mut engine = ActivityEngine::new(repository, settings(), 0, "2026-08-12").unwrap();
        let chrome = ActivitySourceInput {
            kind: ActivitySourceKind::Application,
            executable: "chrome.exe".into(),
            display_name: "Google Chrome".into(),
            domain: None,
            title: None,
            audible: true,
        };
        engine
            .tick(ActivityTick {
                now_epoch_ms: 1_000,
                local_date: "2026-08-12".into(),
                delta_secs: 1.0,
                idle_secs: 400,
                locked_or_sleeping: false,
                break_active: false,
                source: Some(chrome),
            })
            .unwrap();
        assert!(engine.current_session().is_none());
    }

    #[test]
    fn private_exclusions_advance_time_without_storing_identity() {
        let repository = test_repository("private");
        let mut private_settings = settings();
        private_settings.excluded_apps.push("code.exe".into());
        let mut engine =
            ActivityEngine::new(repository.clone(), private_settings, 0, "2026-08-12").unwrap();
        engine
            .tick(ActivityTick {
                now_epoch_ms: 1_000,
                local_date: "2026-08-12".into(),
                delta_secs: 1.0,
                idle_secs: 0,
                locked_or_sleeping: false,
                break_active: false,
                source: Some(app_source()),
            })
            .unwrap();
        engine.flush(1_000).unwrap();
        let review = repository.current_session_review().unwrap().unwrap();
        assert_eq!(review.session.active_secs, 1.0);
        assert!(review.activities.is_empty());
        assert_eq!(review.category_totals.get("privateActivity"), Some(&1.0));
    }

    #[test]
    fn suspended_monotonic_delta_is_not_credited() {
        let repository = test_repository("sleep");
        let mut engine = ActivityEngine::new(repository, settings(), 0, "2026-08-12").unwrap();
        engine
            .tick(ActivityTick {
                now_epoch_ms: 60_000,
                local_date: "2026-08-12".into(),
                delta_secs: 60.0,
                idle_secs: 0,
                locked_or_sleeping: false,
                break_active: false,
                source: Some(app_source()),
            })
            .unwrap();
        assert!(engine.current_session().is_none());
    }
}
