import { siteConfig } from "@/lib/config/site";
import { Logo } from "@/components/SiteElements";

export function Footer() {
    return (
        <footer className="container text-slate-600">
            <div className="flex flex-col items-center justify-between gap-4 border-t border-t-slate-200 py-10 md:h-24 md:flex-row md:py-0">
                <div className="flex flex-col items-center gap-4 px-8 md:flex-row md:gap-2 md:px-0">
                    <Logo w={32} h={32} />
                    <p className="text-center text-sm leading-loose md:text-left">
                        Built by{" "}
                        <a
                            href={siteConfig.links.github}
                            target="_blank"
                            rel="noreferrer"
                            className="font-medium underline underline-offset-4"
                        >
                            Euraxluo
                        </a>
                    </p>
                </div>
            </div>
        </footer>
    )
}
