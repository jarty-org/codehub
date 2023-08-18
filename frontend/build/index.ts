
interface ViteEnv {
    VITE_PORT: number;
    VITE_PUBLIC_PATH: string;
    VITE_ROUTER_HISTORY: string;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type Recordable<T = any> = Record<string, T>;


/** 处理环境变量 */
const warpperEnv = (envConf: Recordable): ViteEnv => {
    /** 此处为默认值 */
    const ret: ViteEnv = {
      VITE_PORT: 3000,
      VITE_PUBLIC_PATH: "",
      VITE_ROUTER_HISTORY: "",
    };
  
    for (const envName of Object.keys(envConf)) {
      let realName = envConf[envName].replace(/\\n/g, "\n");
      realName =
        realName === "true" ? true : realName === "false" ? false : realName;
  
      if (envName === "VITE_PORT") {
        realName = Number(realName);
      }
      ret[envName] = realName;
      if (typeof realName === "string") {
        process.env[envName] = realName;
      } else if (typeof realName === "object") {
        process.env[envName] = JSON.stringify(realName);
      }
    }
    return ret;
  };
  
  export { warpperEnv };