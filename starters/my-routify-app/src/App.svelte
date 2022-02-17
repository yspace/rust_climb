<script>
  import { Router } from "@roxi/routify";
  import { routes } from "../.routify/routes";
</script>

<div class="flex bg-slate-100">
  <a href="/example">example</a>
  <a href="/">index</a>
</div>
<Router {routes} />