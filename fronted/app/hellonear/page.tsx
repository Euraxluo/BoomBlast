
'use client';
import { useWallet } from '@/components/near/WalletSelector';
import { Input } from '@/components/ui/input';
import { ContractId } from '@/lib/config/contract';
import { useEffect, useState } from 'react';


export default function HelloNear() {
    const { accounts, viewMethod, callMethod } = useWallet();
    const [greeting, setGreeting] = useState();
    const [showSpinner, setShowSpinner] = useState(false);

    const [loggedIn, setLoggedIn] = useState(false);


    useEffect(() => {
        viewMethod && viewMethod(ContractId, 'get_greeting', {}).then(
            greeting => setGreeting(greeting)
        );
    }, [viewMethod]);

    const saveGreeting = async () => {
        setShowSpinner(true);
        await callMethod!(ContractId, 'set_greeting', { greeting });
        setShowSpinner(false);
    };

    useEffect(() => {
        const signedAccountId = (accounts?.find((account) => account.active)?.accountId || null);
        console.log('signedAccountId', accounts);
        console.log('signedAccountId', signedAccountId);
        setLoggedIn(!!signedAccountId);
    }, [accounts]);



    return (
        <main className='bg-white'>
            <div >
                <p>
                    Contract: &nbsp;
                    <code >{ContractId}</code>
                </p>
            </div>

            <div >
                <h1 className="w-100"> The contract says: <code>{greeting}</code> </h1>
                <div />
                <div className="input-group">
                    <Input type="email" placeholder="Store a new greeting" onChange={t => { setGreeting(t.target.value); }}/>
                    <div className="input-group-append">
                        <button className="btn btn-secondary" onClick={saveGreeting}>
                            <span hidden={showSpinner}> Save </span>
                            <i className="spinner-border spinner-border-sm" hidden={!showSpinner}></i>
                        </button>
                    </div>
                </div>
                <div className='w-100 text-end align-text-center' hidden={loggedIn}>
                    <p className='m-0'> Please login to change the greeting </p>
                </div>
            </div>
        </main>
    );
}
