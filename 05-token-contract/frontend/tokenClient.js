// ═══════════════════════════════════════════════════════════════════════════════
//                    STELLAR SDK + FREIGHTER INTEGRATION
//                    SEP-41 Token Contract Interaction
// ═══════════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════════
//                              INSTALLATION
// ═══════════════════════════════════════════════════════════════════════════════
/*
Run these commands in your terminal:

npm init -y
npm install @stellar/stellar-sdk @stellar/freighter-api

Or with yarn:
yarn add @stellar/stellar-sdk @stellar/freighter-api
*/

// ═══════════════════════════════════════════════════════════════════════════════
//                              IMPORTS
// ═══════════════════════════════════════════════════════════════════════════════

import * as StellarSdk from '@stellar/stellar-sdk';
const {
    Contract,
    Keypair,
    Networks,
    TransactionBuilder,
    SorobanRpc,
    Address,
    xdr,
    nativeToScVal,
    scValToNative
} = StellarSdk;

// For Freighter wallet integration
import freighter from '@stellar/freighter-api';

// ═══════════════════════════════════════════════════════════════════════════════
//                              CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

const CONFIG = {
    // Network settings
    NETWORK: 'TESTNET',
    NETWORK_PASSPHRASE: Networks.TESTNET,
    RPC_URL: 'https://soroban-testnet.stellar.org',

    // Replace with your deployed contract ID
    CONTRACT_ID: 'CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX', // Your contract

    // For testing without Freighter (use Keypair)
    SECRET_KEY: 'SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX', // Your secret key
};

// Create RPC server connection
const server = new SorobanRpc.Server(CONFIG.RPC_URL);

// ═══════════════════════════════════════════════════════════════════════════════
//                    PART 1: FREIGHTER WALLET INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Check if Freighter wallet is installed
 */
async function isFreighterInstalled() {
    const { isConnected } = await freighter.isConnected();
    return isConnected;
}

/**
 * Get the connected wallet address from Freighter
 */
async function getFreighterAddress() {
    // Check if installed
    if (!await isFreighterInstalled()) {
        throw new Error('Freighter wallet is not installed!');
    }

    // Request access to Freighter
    const { address } = await freighter.getAddress();

    console.log('Connected wallet address:', address);
    return address;
}

/**
 * Get the network from Freighter
 */
async function getFreighterNetwork() {
    const network = await freighter.getNetworkDetails();
    console.log('Connected network:', network);
    return network;
}

/**
 * Sign a transaction using Freighter
 */
async function signWithFreighter(xdrTransaction) {
    const { signedTxXdr } = await freighter.signTransaction(
        xdrTransaction,
        {
            network: CONFIG.NETWORK,
            networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
        }
    );

    return signedTxXdr;
}

// ═══════════════════════════════════════════════════════════════════════════════
//                    PART 2: STELLAR SDK - HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Convert JavaScript values to Soroban ScVal
 * This is how we send data to smart contracts
 */
function jsToScVal(value, type) {
    switch (type) {
        case 'address':
            return new Address(value).toScVal();

        case 'string':
            return nativeToScVal(value, { type: 'string' });

        case 'i128':
            return nativeToScVal(value, { type: 'i128' });

        case 'u32':
            return nativeToScVal(value, { type: 'u32' });

        case 'symbol':
            return nativeToScVal(value, { type: 'symbol' });

        default:
            return nativeToScVal(value);
    }
}

/**
 * Get account details from the network
 */
async function getAccount(publicKey) {
    try {
        return await server.getAccount(publicKey);
    } catch (error) {
        throw new Error(`Account not found: ${publicKey}`);
    }
}

/**
 * Wait for transaction confirmation
 */
async function waitForTransaction(hash) {
    console.log(`Waiting for transaction: ${hash}`);

    let status;
    let attempts = 0;
    const maxAttempts = 30;

    while (attempts < maxAttempts) {
        try {
            const response = await server.getTransaction(hash);
            status = response.status;

            if (status === 'SUCCESS') {
                console.log('Transaction successful!');
                return response;
            } else if (status === 'FAILED') {
                throw new Error('Transaction failed');
            }

            // NOT_FOUND means still processing
            await new Promise(resolve => setTimeout(resolve, 1000));
            attempts++;
        } catch (error) {
            await new Promise(resolve => setTimeout(resolve, 1000));
            attempts++;
        }
    }

    throw new Error('Transaction timeout');
}

// ═══════════════════════════════════════════════════════════════════════════════
//                    PART 3: TOKEN CONTRACT INTERACTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Create a contract instance
 */
function getContract() {
    return new Contract(CONFIG.CONTRACT_ID);
}

// ─────────────────────────────────────────────────────────────────────────────
//                         VIEW FUNCTIONS (No signature needed)
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Get token name
 */
async function getTokenName() {
    const contract = getContract();

    // Build the call
    const result = await server.simulateTransaction(
        new TransactionBuilder(
            await getAccount(CONFIG.CONTRACT_ID), // Any valid account
            {
                fee: '100',
                networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
            }
        )
            .addOperation(contract.call('name'))
            .setTimeout(30)
            .build()
    );

    // Parse result
    const scVal = result.result.retval;
    return scValToNative(scVal);
}

/**
 * Get token symbol
 */
async function getTokenSymbol() {
    const contract = getContract();
    const account = await getAccount((await getFreighterAddress()));

    const tx = new TransactionBuilder(account, {
        fee: '100',
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(contract.call('symbol'))
        .setTimeout(30)
        .build();

    const result = await server.simulateTransaction(tx);
    return scValToNative(result.result.retval);
}

/**
 * Get token decimals
 */
async function getDecimals() {
    const contract = getContract();
    const account = await getAccount((await getFreighterAddress()));

    const tx = new TransactionBuilder(account, {
        fee: '100',
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(contract.call('decimals'))
        .setTimeout(30)
        .build();

    const result = await server.simulateTransaction(tx);
    return scValToNative(result.result.retval);
}

/**
 * Get balance of an address
 */
async function getBalance(address) {
    const contract = getContract();
    const account = await getAccount((await getFreighterAddress()));

    const tx = new TransactionBuilder(account, {
        fee: '100',
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(
            contract.call(
                'balance',
                jsToScVal(address, 'address')
            )
        )
        .setTimeout(30)
        .build();

    const result = await server.simulateTransaction(tx);
    return scValToNative(result.result.retval);
}

/**
 * Get total supply
 */
async function getTotalSupply() {
    const contract = getContract();
    const account = await getAccount((await getFreighterAddress()));

    const tx = new TransactionBuilder(account, {
        fee: '100',
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(contract.call('total_supply'))
        .setTimeout(30)
        .build();

    const result = await server.simulateTransaction(tx);
    return scValToNative(result.result.retval);
}

/**
 * Get allowance
 */
async function getAllowance(owner, spender) {
    const contract = getContract();
    const account = await getAccount((await getFreighterAddress()));

    const tx = new TransactionBuilder(account, {
        fee: '100',
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(
            contract.call(
                'allowance',
                jsToScVal(owner, 'address'),
                jsToScVal(spender, 'address')
            )
        )
        .setTimeout(30)
        .build();

    const result = await server.simulateTransaction(tx);
    return scValToNative(result.result.retval);
}

// ─────────────────────────────────────────────────────────────────────────────
//                         WRITE FUNCTIONS (Needs signature)
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Transfer tokens (using Freighter)
 */
async function transfer(from, to, amount) {
    const contract = getContract();
    const account = await getAccount(from);

    // Build transaction
    let tx = new TransactionBuilder(account, {
        fee: '100000', // Higher fee for contract calls
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(
            contract.call(
                'transfer',
                jsToScVal(from, 'address'),
                jsToScVal(to, 'address'),
                jsToScVal(amount, 'i128')
            )
        )
        .setTimeout(30)
        .build();

    // Simulate first to get the proper transaction
    const simulation = await server.simulateTransaction(tx);

    if (SorobanRpc.Api.isSimulationError(simulation)) {
        throw new Error(`Simulation failed: ${simulation.error}`);
    }

    // Prepare transaction with simulation results
    tx = SorobanRpc.assembleTransaction(tx, simulation).build();

    // Sign with Freighter
    const signedXdr = await signWithFreighter(tx.toXDR());
    const signedTx = TransactionBuilder.fromXDR(signedXdr, CONFIG.NETWORK_PASSPHRASE);

    // Submit transaction
    const response = await server.sendTransaction(signedTx);

    if (response.status === 'ERROR') {
        throw new Error('Transaction submission failed');
    }

    // Wait for confirmation
    return await waitForTransaction(response.hash);
}

/**
 * Approve spender (using Freighter)
 */
async function approve(owner, spender, amount, expirationLedger) {
    const contract = getContract();
    const account = await getAccount(owner);

    let tx = new TransactionBuilder(account, {
        fee: '100000',
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(
            contract.call(
                'approve',
                jsToScVal(owner, 'address'),
                jsToScVal(spender, 'address'),
                jsToScVal(amount, 'i128'),
                jsToScVal(expirationLedger, 'u32')
            )
        )
        .setTimeout(30)
        .build();

    const simulation = await server.simulateTransaction(tx);

    if (SorobanRpc.Api.isSimulationError(simulation)) {
        throw new Error(`Simulation failed: ${simulation.error}`);
    }

    tx = SorobanRpc.assembleTransaction(tx, simulation).build();

    const signedXdr = await signWithFreighter(tx.toXDR());
    const signedTx = TransactionBuilder.fromXDR(signedXdr, CONFIG.NETWORK_PASSPHRASE);

    const response = await server.sendTransaction(signedTx);

    if (response.status === 'ERROR') {
        throw new Error('Transaction submission failed');
    }

    return await waitForTransaction(response.hash);
}

/**
 * Transfer from (using allowance, with Freighter)
 */
async function transferFrom(spender, from, to, amount) {
    const contract = getContract();
    const account = await getAccount(spender);

    let tx = new TransactionBuilder(account, {
        fee: '100000',
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(
            contract.call(
                'transfer_from',
                jsToScVal(spender, 'address'),
                jsToScVal(from, 'address'),
                jsToScVal(to, 'address'),
                jsToScVal(amount, 'i128')
            )
        )
        .setTimeout(30)
        .build();

    const simulation = await server.simulateTransaction(tx);

    if (SorobanRpc.Api.isSimulationError(simulation)) {
        throw new Error(`Simulation failed: ${simulation.error}`);
    }

    tx = SorobanRpc.assembleTransaction(tx, simulation).build();

    const signedXdr = await signWithFreighter(tx.toXDR());
    const signedTx = TransactionBuilder.fromXDR(signedXdr, CONFIG.NETWORK_PASSPHRASE);

    const response = await server.sendTransaction(signedTx);

    if (response.status === 'ERROR') {
        throw new Error('Transaction submission failed');
    }

    return await waitForTransaction(response.hash);
}

/**
 * Mint tokens (Admin only, using Freighter)
 */
async function mint(adminAddress, recipientAddress, amount) {
    const contract = getContract();
    const account = await getAccount(adminAddress);

    let tx = new TransactionBuilder(account, {
        fee: '100000',
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(
            contract.call(
                'mint',
                jsToScVal(recipientAddress, 'address'),
                jsToScVal(amount, 'i128')
            )
        )
        .setTimeout(30)
        .build();

    const simulation = await server.simulateTransaction(tx);

    if (SorobanRpc.Api.isSimulationError(simulation)) {
        throw new Error(`Simulation failed: ${simulation.error}`);
    }

    tx = SorobanRpc.assembleTransaction(tx, simulation).build();

    const signedXdr = await signWithFreighter(tx.toXDR());
    const signedTx = TransactionBuilder.fromXDR(signedXdr, CONFIG.NETWORK_PASSPHRASE);

    const response = await server.sendTransaction(signedTx);

    if (response.status === 'ERROR') {
        throw new Error('Transaction submission failed');
    }

    return await waitForTransaction(response.hash);
}

/**
 * Burn tokens (using Freighter)
 */
async function burn(from, amount) {
    const contract = getContract();
    const account = await getAccount(from);

    let tx = new TransactionBuilder(account, {
        fee: '100000',
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(
            contract.call(
                'burn',
                jsToScVal(from, 'address'),
                jsToScVal(amount, 'i128')
            )
        )
        .setTimeout(30)
        .build();

    const simulation = await server.simulateTransaction(tx);

    if (SorobanRpc.Api.isSimulationError(simulation)) {
        throw new Error(`Simulation failed: ${simulation.error}`);
    }

    tx = SorobanRpc.assembleTransaction(tx, simulation).build();

    const signedXdr = await signWithFreighter(tx.toXDR());
    const signedTx = TransactionBuilder.fromXDR(signedXdr, CONFIG.NETWORK_PASSPHRASE);

    const response = await server.sendTransaction(signedTx);

    if (response.status === 'ERROR') {
        throw new Error('Transaction submission failed');
    }

    return await waitForTransaction(response.hash);
}

// ═══════════════════════════════════════════════════════════════════════════════
//                    PART 4: USING KEYPAIR (Without Freighter)
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Transfer tokens using Keypair (for backend/scripts)
 * This is useful when you don't have Freighter (like in Node.js)
 */
async function transferWithKeypair(fromSecretKey, toAddress, amount) {
    const keypair = Keypair.fromSecret(fromSecretKey);
    const fromAddress = keypair.publicKey();

    const contract = getContract();
    const account = await getAccount(fromAddress);

    let tx = new TransactionBuilder(account, {
        fee: '100000',
        networkPassphrase: CONFIG.NETWORK_PASSPHRASE,
    })
        .addOperation(
            contract.call(
                'transfer',
                jsToScVal(fromAddress, 'address'),
                jsToScVal(toAddress, 'address'),
                jsToScVal(amount, 'i128')
            )
        )
        .setTimeout(30)
        .build();

    const simulation = await server.simulateTransaction(tx);

    if (SorobanRpc.Api.isSimulationError(simulation)) {
        throw new Error(`Simulation failed: ${simulation.error}`);
    }

    tx = SorobanRpc.assembleTransaction(tx, simulation).build();

    // Sign with Keypair instead of Freighter
    tx.sign(keypair);

    const response = await server.sendTransaction(tx);

    if (response.status === 'ERROR') {
        throw new Error('Transaction submission failed');
    }

    return await waitForTransaction(response.hash);
}

// ═══════════════════════════════════════════════════════════════════════════════
//                    PART 5: EXAMPLE USAGE
// ═══════════════════════════════════════════════════════════════════════════════

async function main() {
    try {
        // ───────────────────────────────────────────────────────────
        //                 Check Freighter Connection
        // ───────────────────────────────────────────────────────────

        console.log('=== Checking Freighter ===');

        if (await isFreighterInstalled()) {
            const address = await getFreighterAddress();
            console.log('Freighter connected:', address);
        } else {
            console.log('Freighter not installed. Using Keypair instead.');
        }

        // ───────────────────────────────────────────────────────────
        //                 Read Token Info
        // ───────────────────────────────────────────────────────────

        console.log('\n=== Token Info ===');

        const name = await getTokenName();
        const symbol = await getTokenSymbol();
        const decimals = await getDecimals();
        const totalSupply = await getTotalSupply();

        console.log('Name:', name);
        console.log('Symbol:', symbol);
        console.log('Decimals:', decimals);
        console.log('Total Supply:', totalSupply);

        // ───────────────────────────────────────────────────────────
        //                 Check Balance
        // ───────────────────────────────────────────────────────────

        console.log('\n=== Balance Check ===');

        const myAddress = await getFreighterAddress();
        const balance = await getBalance(myAddress);
        console.log('My balance:', balance);

        // ───────────────────────────────────────────────────────────
        //                 Transfer Tokens
        // ───────────────────────────────────────────────────────────

        console.log('\n=== Transfer ===');

        const recipientAddress = 'GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX';
        const transferAmount = 100n; // Use BigInt for i128

        // Uncomment to execute:
        // await transfer(myAddress, recipientAddress, transferAmount);
        console.log('Transfer would send', transferAmount, 'tokens');

        // ───────────────────────────────────────────────────────────
        //                 Approve and TransferFrom
        // ───────────────────────────────────────────────────────────

        console.log('\n=== Approve ===');

        const spenderAddress = 'GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX';
        const approveAmount = 500n;
        const expiration = 1000000; // Ledger number

        // Uncomment to execute:
        // await approve(myAddress, spenderAddress, approveAmount, expiration);
        console.log('Approval would allow', approveAmount, 'tokens');

    } catch (error) {
        console.error('Error:', error.message);
    }
}

// Run main
main();

// ═══════════════════════════════════════════════════════════════════════════════
//                              EXPORT FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

export {
    // Freighter functions
    isFreighterInstalled,
    getFreighterAddress,
    getFreighterNetwork,
    signWithFreighter,

    // View functions
    getTokenName,
    getTokenSymbol,
    getDecimals,
    getBalance,
    getTotalSupply,
    getAllowance,

    // Write functions
    transfer,
    approve,
    transferFrom,
    mint,
    burn,

    // Keypair version
    transferWithKeypair,
};

/*
╔══════════════════════════════════════════════════════════════════════════════╗
║                    JAVASCRIPT INTEGRATION GUIDE                              ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  FREIGHTER WALLET (Browser):                                                 ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  1. User has Freighter browser extension installed                           ║
║  2. freighter.getAddress() - gets connected wallet                           ║
║  3. freighter.signTransaction() - signs transaction                          ║
║  4. We submit signed transaction to network                                  ║
║                                                                              ║
║  KEYPAIR (Node.js/Backend):                                                  ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  1. Use secret key directly: Keypair.fromSecret(secret)                      ║
║  2. Sign with: tx.sign(keypair)                                              ║
║  3. No browser wallet needed                                                 ║
║                                                                              ║
║  FLOW FOR CONTRACT CALLS:                                                    ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  1. Build transaction with contract.call()                                   ║
║  2. Simulate with server.simulateTransaction()                               ║
║  3. Assemble with SorobanRpc.assembleTransaction()                           ║
║  4. Sign (Freighter or Keypair)                                              ║
║  5. Submit with server.sendTransaction()                                     ║
║  6. Wait for confirmation                                                    ║
║                                                                              ║
║  VALUE CONVERSION:                                                           ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  JavaScript → Soroban: nativeToScVal() or custom jsToScVal()                 ║
║  Soroban → JavaScript: scValToNative()                                       ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
*/
