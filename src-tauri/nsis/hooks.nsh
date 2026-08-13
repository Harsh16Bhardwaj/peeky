!macro NSIS_HOOK_POSTINSTALL
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ; Stop the tray process before removing files or optional local data.
  ; A running Peeky process can keep WebView2 and activity.db handles open.
  nsExec::ExecToLog 'taskkill /F /IM Peeky.exe'
  Pop $0
  Sleep 700
  MessageBox MB_YESNO|MB_DEFBUTTON1 "Delete local Peeky activity history and settings?" IDNO keep_peeky_data
    RMDir /r "$LOCALAPPDATA\Peeky"
  keep_peeky_data:
!macroend

