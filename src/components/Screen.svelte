<script lang="ts">
  import MainNavBar from "./MainNavBar.svelte";
</script>

<div class="flex h-screen">
  <MainNavBar />
  <div class="flex-1 p-4">
    <!-- Main content area -->
    <slot></slot>
  </div>
</div>
