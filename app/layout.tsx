import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import { headers } from "next/headers";
import "./globals.css";

const geistSans = Geist({ variable: "--font-geist-sans", subsets: ["latin"] });
const geistMono = Geist_Mono({ variable: "--font-geist-mono", subsets: ["latin"] });

export async function generateMetadata(): Promise<Metadata> {
  const incoming = await headers();
  const host = incoming.get("host") ?? "localhost:3002";
  const protocol = incoming.get("x-forwarded-proto") ?? (host.startsWith("localhost") ? "http" : "https");
  const origin = `${protocol}://${host}`;

  return {
    metadataBase: new URL(origin),
    title: { default: "Peeky - A calmer way to use your screen", template: "%s | Peeky" },
    description: "A calm Windows break companion with gentle reminders and an optional, fully local activity journal.",
    applicationName: "Peeky",
    keywords: ["break reminder", "eye care", "Windows", "posture reminder", "local activity journal"],
    icons: { icon: "/peeky-icon.png", shortcut: "/peeky-icon.png", apple: "/peeky-icon.png" },
    openGraph: {
      title: "Peeky - Your screen is intense. Peeky isn't.",
      description: "Gentle break reminders and a private, local activity journal for Windows.",
      type: "website",
      images: [{ url: `${origin}/og.png`, width: 1200, height: 630, alt: "Peeky - Your screen is intense. Peeky isn't." }],
    },
    twitter: {
      card: "summary_large_image",
      title: "Peeky - A calmer way to use your screen",
      description: "Gentle breaks. Local context. Zero cloud.",
      images: [`${origin}/og.png`],
    },
  };
}

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="en"><body className={`${geistSans.variable} ${geistMono.variable}`}>{children}</body></html>;
}
