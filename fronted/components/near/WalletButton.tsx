'use client';

import { useEffect, useState } from "react";
import { useWallet } from "@/components/near/WalletSelector";
import { Button } from "@/components/ui/button";
export default function WalletButton() {
    const { logOut, logIn, accounts } = useWallet();
    const [action, setAction] = useState(() => { });
    const [signedAccountId, setSignedAccountId] = useState();
    const [label, setLabel] = useState('Loading...');
    useEffect(() => {
        console.log('WalletButton useEffect.');
        setSignedAccountId(accounts?.find((account) => account.active)?.accountId || null);
        if (signedAccountId) {
            setAction(() => logOut);
            setLabel(`Logout ${signedAccountId}`);
        } else {
            setAction(() => logIn);
            setLabel('Login');
        }
    }, [accounts, logIn, logOut, setAction]);

    return (
        <Button
            className='text-black  bg-[linear-gradient(90deg,#CBFF01_0.13%,#00FFA3_99.92%)]'
            onClick={action!}
        >
            {label}
        </Button>
    );
};