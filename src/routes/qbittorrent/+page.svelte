<script lang="ts">
  import { onMount } from "svelte";
  import { createChildWebview } from "$lib/utils";

  onMount(async () => {
    await createChildWebview("qbittorrent", "localhost:8888/");
  });
</script>
