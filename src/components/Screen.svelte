<script lang="ts">
  let { children } = $props();
</script>

<div class="flex h-screen">
  <div class="flex-1 p-4">
    <!-- Main content area -->
    {@render children()}
  </div>
</div>
