import { absoluteUrl } from "@/lib/utils"

export type SiteConfig = {
  logo: string
  name: string
  description: string
  url: string
  ogImage: string
  author: string
  links: {
    github: string
  }
}

export const siteConfig: SiteConfig = {
  logo: "/quest_192x192.png",
  name: "Quest",
  description: "Quest challenge rewards",
  url: absoluteUrl(),
  ogImage: "/quest_512x512.png",
  author: "Euaxluo",
  links: {
    github: "https://github.com/Euraxluo",
  },
}
