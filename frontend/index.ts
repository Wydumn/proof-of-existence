// Import
import { ApiPromise, WsProvider } from '@polkadot/api';
import "@polkadot/api-augment";

// Construct
const WEB_SOCKET = "ws://127.0.0.1:9944"
const connectSubstrate = async () => {
  const wsProvider = new WsProvider(WEB_SOCKET);
  const api = await ApiPromise.create({ provider: wsProvider });
  await api.isReady;
  console.log("connection to substrate is OK.");
  return api;
};

const getOffchainStorage = async (api: ApiPromise) => {
  const value = api.rpc.offchain.localStorageGet('PERSISTENT', '0x6b697474795f70616c6c65743a3a696e646578696e67312f16000000');
  return value;
}

const main = async () => {
  const api = await connectSubstrate();
  // read offchain storage
  const res = await getOffchainStorage(api);
  const resStr = res.toHuman();

  console.log("localStorageGet result", resStr);
};

main()
.then(() => {
  console.log("successfully exited");
  process.exit(0);
})
.catch((err) => {
  console.log("error occur:", err);
  process.exit(1);
});