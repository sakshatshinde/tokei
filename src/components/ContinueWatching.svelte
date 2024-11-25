<script lang="ts">
  interface Show {
    id: string; // Unique identifier for the show (e.g., a database ID or external API ID)
    title: string; // Title of the show
    Season: string;
    posterUrl: string; // URL of the show's poster image
    totalEpisodes: number; // Total number of episodes in the show
    lastWatchedEpisode: {
      episodeNumber: number | null; // Episode number last watched (null if none)
      timestamp: number | null; // Timestamp (milliseconds since epoch) of last watch
    };
  }

  const dummyShows: Show[] = [
    {
      id: "show1",
      title: "Toradora",
      Season: "1",
      posterUrl: "https://pic.re/image",
      totalEpisodes: 10,
      lastWatchedEpisode: {
        episodeNumber: 5,
        timestamp: 1678886400000, // Example timestamp (March 15, 2024)
      },
    },
    {
      id: "show2",
      title: "Attack on Titan",
      Season: "2",
      posterUrl:
        "https://platform.polygon.com/wp-content/uploads/sites/2/2024/08/filters_quality95formatwebp.webp",
      totalEpisodes: 87,
      lastWatchedEpisode: {
        episodeNumber: 1,
        timestamp: null,
      },
    },
  ];

  function resumeShow(show: Show) {
    // Implement your show resuming logic here.  This might involve
    // navigating to a specific episode in your app, sending an event
    // to your Tauri backend, etc.
    console.log(
      `Resuming show: ${show.title} (from episode ${show.lastWatchedEpisode || 1})`
    );
  }
</script>

<h1 class="text-2xl mb-4 font-semibold">Continue watching</h1>

<div class="flex flex-wrap gap-4 p-2">
  {#each dummyShows as show}
    <div
      class="card image-full w-96 shadow-xl hover:scale-105 will-change-transform transform transition duration-200"
    >
      <figure>
        <img src={show.posterUrl} alt={show.title} class="opacity-100" />
      </figure>
      <div class="card-body card-info">
        <h2 class="card-title p-2 font-bold justify-end">{show.title}</h2>
        <div class="card-actions justify-end">
          {#if show.lastWatchedEpisode.episodeNumber !== null}
            <div class="badge justify-end font-bold">
              S{show.Season} | EP{show.lastWatchedEpisode.episodeNumber + 1}
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/each}
</div>

<style>
  .card {
    position: relative;
    display: card;
    width: 24rem;
    image-resolution: full;
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -1px rgba(0, 0, 0, 0.06);
    transition: transform 0.2s ease-in-out;
  }

  .card:hover {
    transform: scale(1.05);
  }

  .card::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: radial-gradient(
      circle at top left,
      rgba(255, 255, 255, 0.3),
      transparent 70%
    );
    opacity: 0;
    transition: opacity 0.3s ease-in-out;
  }

  .card:hover::before {
    opacity: 1;
  }

  .card-info {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: flex-end;
    justify-content: flex-end;
    padding: 1rem;
    background: linear-gradient(to bottom, transparent, rgba(0, 0, 0, 0.6));
    opacity: 1;
    transition: opacity 0.2s ease-in-out;
  }

  .card:hover {
    opacity: 1;
  }
</style>
