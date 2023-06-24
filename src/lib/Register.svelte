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
  import { responseStore, userStore } from "./stores";
  let username = "";
  let password = "";
  let res: Response = {
    message: "",
    user: {
      username: "",
      id: 0,
      license: "",
      licenseExpiry: "",
    },
    error: "",
  };
  let user: User = {
    username: "",
    id: 0,
    license: "",
    licenseExpiry: "",
  };
  listen("licenseExpired", (event) => {
    console.log(event);
  });

  async function register() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    res = await invoke("register", { username, password });
    console.log(res);
    user = res.user as User;
    userStore.set(user);
    responseStore.set(res);
    console.log(user);
  }
</script>

<div class="">
  <form
    class="flex-col flex justify-center items-center"
    on:submit|preventDefault={register}
  >
    <input
      id="username-input"
      class="w-max bg-gray-700 caret-white text-gray-100"
      placeholder="Username"
      bind:value={username}
    />
    <div class="p-2" />
    <input
      id="password-input"
      class="w-max bg-gray-700 caret-white text-gray-100"
      placeholder="Password"
      bind:value={password}
      type="password"
    />
    <div class="p-2" />

    <button
      type="submit"
      class="w-max bg-gray-800 text-white hover:bg-gray-700 transition-all duration-150 active:border-indigo-950"
      >Register</button
    >
  </form>

  {#if $responseStore && $responseStore.error !== ""}
    <h1 class="text-lg text-red-500">{$responseStore.message}</h1>
    <h1 class="text-white">{$responseStore.error}</h1>
  {/if}
  {#if $userStore && $userStore.id > 0}
    <div>
      <h1 class="text-lg text-green-500">{$responseStore.message}</h1>
      <h2>{$responseStore.error}</h2>
    </div>
    <div>
      <h1 class="text-lg text-green-500">{$userStore.username}</h1>
    </div>
  {/if}
</div>
