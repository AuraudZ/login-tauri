<script script lang="ts">
  interface User {
    username: string;
    id: number;
    license: string;
    licenseExpiry: string;
  }

  interface Response {
    message: string;
    user: User;
    error: string;
  }

  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  let username = "";
  let password = "";
  let res: Response;
  let user: User = {
    username: "",
    id: 0,
    license: "",
    licenseExpiry: "",
  };
  listen("licenseExpired", (event) => {
    console.log(event);
  });

  async function login() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    res = await invoke("login", { username, password });
    console.log(res);
    if (res.error !== "") {
      user = {
        username: "",
        id: 0,
        license: "",
        licenseExpiry: "",
      };
      return;
    }
    user = res.user as User;
    console.log(user);
  }
</script>

<div class="bg-transparent">
  <form class="flex-col" on:submit|preventDefault={login}>
    <input
      id="username-input"
      placeholder="Enter a name..."
      bind:value={username}
    />
    <input
      id="password-input"
      placeholder="Enter a password"
      bind:value={password}
      type="password"
    />
    <button type="submit">Login</button>
  </form>
  {#if res && res.error !== ""}
    <h1 class="text-lg text-red-500">{res.message}</h1>
    <h2>{res.error}</h2>
  {/if}
  {#if user.id > 0}
    <div>
      <h1>Username: {user.username}</h1>
      <h1>License: {user.license}</h1>
      <h1>License Expiry: {new Date(user.licenseExpiry).toLocaleString()}</h1>
    </div>
  {/if}
</div>
