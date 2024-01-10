'use client';
import { create as createStore, useStore } from 'zustand';
import type { ReactNode } from "react";
import React, { useCallback, useContext, useEffect, useState, createContext } from "react";
import { distinctUntilChanged, map } from "rxjs";
import { providers } from 'near-api-js';

// @ts-ignore
import type { AccountState, WalletSelector } from "@near-wallet-selector/core";
// @ts-ignore
import { setupWalletSelector } from "@near-wallet-selector/core";
import type { WalletSelectorModal } from "@near-wallet-selector/modal-ui";
import { setupModal } from "@near-wallet-selector/modal-ui";
import { setupNearWallet } from "@near-wallet-selector/near-wallet";
import { setupHereWallet } from "@near-wallet-selector/here-wallet";
import { setupSender } from "@near-wallet-selector/sender";
import { setupBitgetWallet } from "@near-wallet-selector/bitget-wallet";
import { setupMyNearWallet } from "@near-wallet-selector/my-near-wallet";
import { setupLedger } from "@near-wallet-selector/ledger";

export type TViewMethod = (
    contractId: string,
    method: string,
    args?: Record<string, any>
) => Promise<any>;

export type TCallMethod = (
    contractId: string,
    method: string,
    args?: Record<string, any>,
    gas?: string,
    deposit?: number,
) => Promise<any>;

export interface WalletStore {
    selector: WalletSelector;
    setStoreSelector: ({ selector }: { selector: WalletSelector }) => void;

    modal: WalletSelectorModal | null;
    setStoreModal: ({ modal }: { modal: WalletSelectorModal }) => void;

    accounts?: Array<AccountState>;
    setStoreAccounts: ({ accounts }: { accounts: Array<AccountState> }) => void;

    logOut?: (() => Promise<void>) | null;
    logIn?: (() => void) | null;
    setStoreLogActions: ({ logOut, logIn }: { logOut: () => Promise<void>, logIn: () => void }) => void;

    viewMethod: TViewMethod | null;
    callMethod: TCallMethod | null;
    setStoreMethods: ({ viewMethod, callMethod }: { viewMethod: WalletStore['viewMethod'], callMethod: WalletStore['callMethod'] }) => void;
}
export const useWallet = createStore<WalletStore>(set => ({
    selector: undefined,
    setStoreSelector: ({ selector }) => set({ selector }),
    modal: null,
    setStoreModal: ({ modal }) => set({ modal }),
    accounts: [],
    setStoreAccounts: ({ accounts }) => set({ accounts }),
    logOut: null,
    logIn: null,
    setStoreLogActions: ({ logOut, logIn }) => set({ logOut, logIn }),
    viewMethod: null,
    callMethod: null,
    setStoreMethods: ({ viewMethod, callMethod }) => set({ viewMethod, callMethod }),
}));

const WalletStoreContext = createContext(useWallet.getState());

export const WalletStoreContextProvider: React.FC<{
    children: ReactNode;
    contractId: string;
    networkId: string
}> = ({ children, contractId, networkId }) => {
    const setStoreSelector = useWallet(store => store.setStoreSelector);
    const [selector, setSelector] = useState<WalletSelector | null>(null);

    const setStoreModal = useWallet(store => store.setStoreModal);
    const [modal, setModal] = useState<WalletSelectorModal | null>(null);

    const setStoreLogActions = useWallet(store => store.setStoreLogActions);

    const setStoreAccounts = useWallet(store => store.setStoreAccounts);
    const [accounts, setAccounts] = useState<Array<AccountState>>([]);

    const setStoreMethods = useWallet(store => store.setStoreMethods);

    const init = useCallback(async () => {
        console.log("WalletStoreContextProvider Init.")
        const _selector = await setupWalletSelector({
            network: networkId,
            debug: true,
            modules: [
                setupMyNearWallet(),
                setupLedger(),
                setupNearWallet(),
                setupSender(),
                setupBitgetWallet(),
                setupHereWallet(),
            ],
        });
        setSelector(_selector);
        setStoreSelector({ selector: _selector });

        const _modal = setupModal(_selector, {
            contractId: contractId,
        });
        setModal(_modal);
        setStoreModal({ modal: _modal });


        const state = _selector.store.getState();
        setAccounts(state.accounts);
        setStoreAccounts(state.accounts);
        console.log("accounts", accounts)
    }, []);

    useEffect(() => {
        init().catch((err) => {
            console.error(err);
            alert("Failed to initialise wallet selector.\nView the console for details.");
        });
    }, [init]);


    useEffect(() => {
        console.log("WalletStoreContextProvider setup Accounts.")
        if (!selector) {
            return;
        }

        const subscription = selector.store.observable
            .pipe(
                map((state: any) => state.accounts),
                distinctUntilChanged(),
            )
            .subscribe((nextAccounts: React.SetStateAction<AccountState[]>) => {
                setAccounts(nextAccounts);
                // @ts-ignore
                const actualAccounts = typeof nextAccounts === 'function' ? nextAccounts((prevState) => prevState) : nextAccounts as AccountState[];
                setStoreAccounts({ accounts: actualAccounts });
            });

        const onHideSubscription = modal!.on("onHide", ({ hideReason }) => {
            console.log(`The reason for hiding the modal ${hideReason}`);
        });

        return () => {
            subscription.unsubscribe();
            onHideSubscription.remove();
        };
    }, [selector, modal]);

    useEffect(() => {
        console.log("WalletStoreContextProvider setup LogActions.")
        if (!selector) {
            return;
        }
        const _logOut = async () => {
            console.log("log out");
            const wallet = await (await selector).wallet();
            wallet.signOut().catch((err: any) => {
                console.log("Failed to sign out");
                console.error(err);
            });
        };
        const _logIn = async () => {
            const _modal = setupModal(selector, {
                contractId: contractId,
            });
            setModal(_modal);
            setStoreModal({ modal: _modal });
            _modal.show();
        };
        setStoreLogActions({ logOut: _logOut, logIn: _logIn });
    }, [selector]);

    useEffect(() => {
        console.log("WalletStoreContextProvider setup Methods.")
        if (!selector) return;

        const viewMethod: TViewMethod = async (contractId, method, args = {}) => {
            const { network } = (await selector).options;
            // const provider = new providers.JsonRpcProvider({ url: network.nodeUrl });
            return new providers.JsonRpcProvider({ url: network.nodeUrl })
                .query({
                    request_type: "call_function",
                    account_id: contractId,
                    method_name: method,
                    args_base64: Buffer.from(JSON.stringify(args)).toString("base64"),
                    finality: "optimistic",
                })
                .then((res) =>
                    // @ts-ignore
                    JSON.parse(Buffer.from(res.result).toString()),
                );
        };
        const callMethod: TCallMethod = async (contractId, method, args = {}, gas = '30000000000000', deposit = 0) => {
            const wallet = await (await selector).wallet();
            const outcome = await wallet.signAndSendTransaction({
                receiverId: contractId,
                actions: [
                    {
                        type: 'FunctionCall',
                        params: {
                            methodName: method,
                            args,
                            gas,
                            deposit,
                        },
                    },
                ],
            });

            return providers.getTransactionLastResult(outcome);
        };

        setStoreMethods({ viewMethod, callMethod });
    }, [selector, setStoreMethods]);

    return (
        <WalletStoreContext.Provider value={useWallet.getState()}>
            {children}
        </WalletStoreContext.Provider>
    )
}