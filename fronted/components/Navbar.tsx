import { Logo, Site } from "@/components/SiteElements";
import WalletButton from "@/components/near/WalletButton";

export function Navbar() {
    return (
        <>
            <Logo w={56} h={56} />
            <Site />
            <WalletButton />
        </>
    );
}
