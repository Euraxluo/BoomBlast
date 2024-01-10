import '@/app/globals.css'
import '@near-wallet-selector/modal-ui/styles.css';

import type { Metadata, Viewport } from 'next'
import { siteConfig } from '@/lib/config/site'
import { Footer } from '@/components/Footer';
import { Header } from '@/components/Header';
import { ContractId, NetworkId } from '@/lib/config/contract';
import { WalletStoreContextProvider } from '@/components/near/WalletSelector';

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  maximumScale: 1,
  themeColor: [
    { media: "(prefers-color-scheme: dark)", color: "black" },
    { media: "(prefers-color-scheme: light)", color: "white" },
  ],
}

export const metadata: Metadata = {
  metadataBase: new URL(siteConfig.url),
  creator: siteConfig.author,
  manifest: '/site.webmanifest',
  title: {
    absolute: siteConfig.name,
    template: `%s | ${siteConfig.name}`,
  },
  description: siteConfig.description,
  keywords: [
    "Quest",
    "Challenges",
    "Rewards",
    "web3",
    "NFT",
    "Coins",
  ],
  authors: [
    {
      name: siteConfig.author,
      url: siteConfig.links.github,
    }
  ],
  openGraph: {
    url: siteConfig.url,
    type: "website",
    title: siteConfig.name,
    description: siteConfig.description,
    images: [
      {
        url: siteConfig.ogImage,
        width: 512,
        height: 512,
        alt: siteConfig.name,
        type: 'image/png',
      },
    ],
    locale: "zh_CN",
    siteName: siteConfig.name,

  },
  icons: {
    icon: "/quest_32x32.png",
    shortcut: "/quest_16x16.png",
    apple: "/quest_32x32.png",
  },
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <WalletStoreContextProvider contractId={ContractId} networkId={NetworkId}>
          <div className="flex flex-col min-h-screen bg-zinc-950">
            <Header />
            <main className="flex-1 p-6 overflow-hidden">{children}</main>
            <Footer />
          </div>
        </WalletStoreContextProvider>
      </body>
    </html >
  )
}
