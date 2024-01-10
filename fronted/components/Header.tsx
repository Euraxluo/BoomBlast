import { Navbar } from "@/components/Navbar";

export function Header() {
    return (
        <header className="container sticky top-0 z-40">
            <div className="flex h-16 items-center justify-between border-b border-b-slate-200 py-4">
                <Navbar />
            </div>
        </header>
    )
}
