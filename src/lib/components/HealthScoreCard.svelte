<script lang="ts">
  interface Props {
    healthScore: number;
    recommendationCount: number;
  }

  let { healthScore, recommendationCount }: Props = $props();
</script>

<div class="panel rounded-[24px] border-white/10 p-5">
  <p class="mono text-[11px] uppercase tracking-[0.24em] text-[color:var(--muted)]">Health score</p>
  <div class="mt-4 flex items-end justify-between gap-4">
    <p class="text-6xl font-bold leading-none">{healthScore}</p>
    <div class="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-[color:var(--muted)]">
      {recommendationCount} recommendations
    </div>
  </div>
  <div class="mt-5 h-3 overflow-hidden rounded-full bg-white/8">
    <div class="h-full rounded-full bg-[linear-gradient(90deg,var(--accent),var(--accent-2))]" style={`width:${healthScore}%`}></div>
  </div>
</div>
