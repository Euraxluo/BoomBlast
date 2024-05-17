import { siteConfig } from '@/lib/config/site'
import Link from 'next/link'

export function Logo({ w, h }: { w: number, h: number }) {
    return (
        <img
            src={siteConfig.logo}
            width={w}
            height={h}
        >
        </img>
    )
}

export function Site() {
    return (
        <Link className="text-white text-base font-semibold leading-6 self-center grow whitespace-nowrap my-auto" href={siteConfig.url}>
            Quest
        </Link>
    )
}
