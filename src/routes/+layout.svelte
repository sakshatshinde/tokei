<script>
  // @ts-ignore
  import MainNavBar from "../components/MainNavBar.svelte";
  import Screen from "../components/Screen.svelte";
  let { children } = $props();
</script>

<div class="min-h-screen flex">
  <!-- Sidebar/Navbar with fixed width -->
  <MainNavBar />

  <!-- Main content area that takes remaining width and height -->
  <main class="flex-1 h-full overflow-auto">
    <Screen>{@render children()}</Screen>
  </main>
</div>
