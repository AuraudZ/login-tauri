<script script lang="ts">
  import { type User, type Response, userStore, responseStore } from "./stores";

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

  // Format the date in the days remaining format
  function formatDate(date: Date) {
    const now = new Date();
    const diff = date.getTime() - now.getTime();
    const days = Math.ceil(diff / (1000 * 60 * 60 * 24));
    return days;
  }

  listen("licenseExpired", (event) => {
    console.log(event);
  });

  async function login() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    res = await invoke("login", { username, password });
    console.log(res);
    responseStore.set(res);
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
    userStore.set(user);
    console.log(user);
  }
</script>

<div>
  <form
    class="flex-col flex justify-center items-center"
    on:submit|preventDefault={login}
  >
    <input
      id="username-input"
      class="w-max bg-gray-700 caret-white text-gray-100"
      placeholder="Enter a name..."
      bind:value={username}
    />
    <div class="p-2" />
    <input
      id="password-input"
      class="w-max bg-gray-700 caret-white text-gray-100"
      placeholder="Enter a password"
      bind:value={password}
      type="password"
    />
    <div class="p-2" />
    <button
      type="submit"
      class="w-max bg-gray-800 text-white hover:bg-gray-700 transition-all duration-150 active:border-indigo-950"
      >Login</button
    >
  </form>

  {#if res && res.error !== ""}
    <h1 class="text-lg text-red-500">{res.message}</h1>
    <h2>{res.error}</h2>
  {/if}
  {#if user.id > 0}
    <div class=" flex-col flex justify-center items-center">
      <h1>Username: {$userStore.username}</h1>
      <h1>License: {$userStore.license}</h1>
      <h1>
        License Expiry: {formatDate(
          new Date(Number($userStore.licenseExpiry) * 1000)
        )} days remaining
      </h1>
      <button
        class="w-max bg-gray-800 text-white hover:bg-gray-700 transition-all duration-150 active:border-indigo-950"
        on:click={() => invoke("logout")}>Logout</button
      >
      <button
        class="w-max bg-gray-800 text-white hover:bg-gray-700 transition-all duration-150 active:border-indigo-950"
        on:click={() => invoke("renew_license")}>Buy more time</button
      >
    </div>
  {/if}
</div>
