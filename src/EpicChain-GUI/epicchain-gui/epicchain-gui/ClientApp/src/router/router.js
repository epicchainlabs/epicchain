import React from 'react';
import { BrowserRouter, Route, Routes, Navigate, Outlet } from 'react-router-dom';
import Home from '../components/home'
import Sync from '../components/sync';

import Chain from '../components/Chain/chain';
import Chainlayout from '../components/Chain/chainlayout';
import BlockDetail from '../components/Chain/blockdetail';
import Chaintrans from '../components/Chain/trans';
import ChainAsset from '../components/Chain/asset';
import AssetDetail from '../components/Chain/assetdetail';

import Walletlayout from '../components/Wallet/walletlayout';
import Walletlist from '../components/Wallet/walletlist';
import Walletdetail from '../components/Wallet/walletdetail';
import Wallettrans from '../components/Wallet/trans';

import Selecttrans from '../components/Transaction/selecttrans';
import Transdetail from '../components/Transaction/transdetail';
import Untransdetail from '../components/Transaction/untransdetail';


import Contract from '../components/Contract/contract';
import Contractlayout from '../components/Contract/contractlayout';
import Contractdeploy from '../components/Contract/deploy';
import ContractUpgrade from '../components/Contract/upgrade';
import Contractinvoke from '../components/Contract/invoke';
import Contractdetail from '../components/Contract/contractdetail';

import Advanced from '../components/Advanced/advanced';
import Advancedlayout from '../components/Advanced/advancedlayout';
import Advancedvote from '../components/Advanced/vote';
import Advancedcandidate from '../components/Advanced/candidate';
import Advancedsignature from '../components/Advanced/signature';
import Advancedcommittee from '../components/Advanced/committee';
import Advanceddesignrole from '../components/Advanced/designrole';
import Advancednoderole from '../components/Advanced/noderole';

import { Layout } from 'antd';

const BasicRoute = () => (
    <BrowserRouter>
        <Routes>
            <Route exact path="/" element={<Home />} />
            <Route exact path="/sync" element={<Sync />} />
            <Route path="/chain" element={<Layout style={{ height: 'calc( 100vh )' }}><Chainlayout /><Outlet /></Layout>}>
                <Route exact path="" element={<Chain />} />
                <Route exact path="detail:height" element={<BlockDetail />} />
                <Route exact path="transaction" element={<Chaintrans />} />
                <Route exact path="transaction:hash" element={<Transdetail />} />
                <Route exact path="untransaction:hash" element={<Untransdetail />} />
                <Route exact path="asset" element={<ChainAsset />} />
                <Route exact path="asset:hash" element={<AssetDetail />} />
            </Route>
            <Route path="/wallet" element={<div><Layout style={{ height: 'calc( 100vh )' }}><Walletlayout /><Outlet /></Layout></div>}>
                <Route exact path="walletlist" element={<Walletlist />} />
                <Route exact path="walletlist:address" element={<Walletdetail />} />
                <Route exact path="address:address" element={<Walletdetail />} />
                <Route exact path="transaction" element={<Wallettrans />} />
                <Route exact path="transaction:hash" element={<Transdetail />} />
                <Route exact path="untransaction:hash" element={<Untransdetail />} />
                <Route exact path="transfer" element={<Selecttrans />} />
            </Route>
            <Route path="/contract" element={<div><Layout style={{ height: 'calc( 100vh )' }}><Contractlayout /><Outlet /></Layout></div>}>
                <Route exact path="" element={<Contract />} />
                <Route exact path="detail:hash" element={<Contractdetail />} />
                <Route exact path="deploy" element={<Contractdeploy />} />
                <Route exact path="upgrade" element={<ContractUpgrade />} />
                <Route exact path="invoke" element={<Contractinvoke />} />
            </Route>
            <Route path="/advanced" element={<div><Layout style={{ height: 'calc( 100vh )' }}><Advancedlayout /><Outlet /></Layout></div>}>
                <Route exact path="" element={<Advanced />} />
                <Route exact path="vote" element={<Advancedvote />} />
                <Route exact path="candidate" element={<Advancedcandidate />} />
                <Route exact path="signature" element={<Advancedsignature />} />
                <Route exact path="committee" element={<Advancedcommittee />} />
                <Route exact path="designrole" element={<Advanceddesignrole />} />
                <Route exact path="getnoderole" element={<Advancednoderole />} />
            </Route>
            <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
    </BrowserRouter>
);

export default BasicRoute;