<template>
  <div class="mise-landing">

    <div class="code-sections">
      <section class="two-column">
        <div class="content-column">
          <h2>Dev Tools</h2>
          <p class="section-description">
            Set up the tools you need for your project with a single command.
          </p>
          <a href="/dev-tools/" class="learn-more">Learn more →</a>
        </div>
        <div class="code-column">
          <div class="code-block with-shell-prompt interactive" @mouseenter="showGif = true" @mouseleave="showGif = false">
            <div v-if="showGif" class="demo-gif">
              <div class="gif-placeholder">
                <div class="loading-animation"></div>
                <img src="/demo.gif" >
              </div>
            </div>
            <pre v-else><code>mise use node@22 python@3 go@latest
# mise go@1.x.x ✓ installed
# mise node@22.x.x ✓ installed
# mise python@3.x.x ✓ installed</code></pre>
          </div>

          <div class="tab-group">
            <div class="tab-header">
              <span class="tab-dot"></span>
              <span>mise.toml</span>
            </div>
            <div class="code-block">
    <pre><code><span class="section">[tools]</span>
<span class="key">node</span> = <span class="string">"22"</span>
<span class="key">python</span> = <span class="string">"3"</span>
<span class="key">go</span> = <span class="string">"latest"</span></code></pre>
            </div>
          </div>
        </div>
      </section>

      <section class="two-column">
        <div class="content-column">
          <h2>Environment Management</h2>
          <p class="section-description">
            Easily manage environment variables per project, similar to direnv but with more features.
          </p>
          <a href="/environments/" class="learn-more">Learn more →</a>
        </div>
        <div class="code-column">
          <div class="code-block with-shell-prompt">
            <pre><code>mise env set FOO=bar</code></pre>
          </div>
          <div class="code-block with-shell-prompt" style="margin-top: 5px;">
            <pre><code>echo $FOO
# bar</code></pre>
          </div>
          <div class="tab-group">
            <div class="tab-header">
              <span class="tab-dot"></span>
              <span>mise.toml</span>
            </div>
            <div class="code-block">
              <pre><code><span class="section">[env]</span>
<span class="key">FOO</span> = <span class="string">"bar"</span></code></pre>
            </div>

          </div>
        </div>
      </section>

      <section class="two-column">
        <div class="content-column">
          <h2>Task Runner</h2>
          <p class="section-description">
            Replace complex build tools with an easy-to-use task runner that leverages your tools and environment variables.
          </p>
          <a href="/tasks" class="learn-more">Learn more →</a>
        </div>
        <div class="code-column">
          <div class="tab-group">
            <div class="tab-header">
              <span class="tab-dot"></span>
              <span>mise.toml</span>
            </div>
            <div class="code-block">
    <pre><code><span class="section">[task.test]</span>
<span class="key">run</span> = <span class="string">"echo 'running tests...'"</span>

<span class="section">[tasks.my_task]</span>
<span class="key">run</span> = [
    <span class="string">"node -e 'console.log(process.version)'"</span>,
    <span class="string">"node -e 'console.log(process.env.FOO)'"</span>
]
<span class="key">depends</span> = <span class="bracket">[</span><span class="string">"test"</span><span class="bracket">]</span></code></pre>
            </div>
          </div>

          <div class="code-block with-shell-prompt">
            <pre><code>mise tasks ls
# my_task     ~/.my-project/mise.toml
# test        ~/.my-project/mise.toml</code></pre>
          </div>
          <div class="code-block with-shell-prompt" style="margin-top: 5px;">
            <pre><code>mise run my_task
# [test] running tests...
# [my_task] v22.x.x
# [my_task] bar</code></pre>
          </div>
        </div>
      </section>

      <section class="two-column">
        <div class="content-column">
          <h2>mise works with your existing setup</h2>
          <p class="section-description">
             Use existing configuration files like .nvmrc, .ruby-version, .tool-versions, .python-version, .sdkmanrc, etc. with mise.<br /><br />
             Import `.env` files.<br /><br/>
             Run file tasks
          </p>
          <a href="/tasks" class="learn-more">Learn more →</a>
        </div>
        <div class="content-column">
        <div class="compatibility-grid">
          <div class="tab-group">
            <div class="tab-header">
              <span class="tab-dot"></span>
              <span>.nvmrc</span>
            </div>
            <div class="code-block">
              <pre><code>22</code></pre>
            </div>
          </div>
          <div class="tab-group">
            <div class="tab-header">
              <span class="tab-dot"></span>
              <span>.python-version</span>
            </div>
            <div class="code-block">
              <pre><code>3.13</code></pre>
            </div>
          </div>
          <div class="tab-group">
            <div class="tab-header">
              <span class="tab-dot"></span>
              <span>.tool-versions</span>
            </div>
            <div class="code-block">
              <pre><code>go 1.23.4</code></pre>
            </div>
          </div>
          <div class="tab-group">
            <div class="tab-header">
              <span class="tab-dot"></span>
              <span>.sdkmanrc</span>
            </div>
            <div class="code-block">
              <pre><code>java=17.0.2.hs-adpt</code></pre>
            </div>
          </div>
          <div class="tab-group">
            <div class="tab-header">
              <span class="tab-dot"></span>
              <span>.ruby-version</span>
            </div>
            <div class="code-block">
              <pre><code>3</code></pre>
            </div>
          </div>
        </div>
        </div>
      </section>
    </div>

    <div class="cta">
      <a href="/getting-started" class="button primary large">Get Started with mise</a>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
const showGif = ref(false)
</script>

<style scoped>
.mise-landing {
  max-width: var(--vp-layout-max-width, 1200px);
  margin: 0 auto;
  padding: 3rem 1.5rem;
}

.two-column {
  display: grid;
  grid-template-columns: 1fr 1.5fr;
  gap: 3rem;
  margin-bottom: 4rem;
  align-items: start;
}

.content-column {
  padding-top: 1rem;
}

.section-description {
  color: var(--vp-c-text-2);
  font-size: 1.1rem;
  line-height: 1.6;
  margin-bottom: 1.5rem;
}

.learn-more {
  color: var(--vp-c-text-3);
  font-size: 0.9rem;
  margin-top: 1rem;
  text-decoration: none;
}

.code-column {
  position: relative;
}

.code-block {
  background-color: var(--vp-code-block-bg);
  padding: 1rem;
  border-radius: 8px;
  overflow: hidden;
  transition: transform 0.3s ease;
  position: relative;
}

.code-block.interactive {
  cursor: pointer;
}

.code-block.interactive:hover {
  transform: scale(1.02);
}

.code-block pre {
  margin: 0;
  font-family: var(--vp-font-family-mono);
  font-size: 0.9rem;
  line-height: 1.5;
  color: var(--vp-c-text-1);
}

.code-block.with-shell-prompt pre code {
  position: relative;
  padding-left: 125px;
}

.code-block.with-shell-prompt pre code::before {
  content: "~/my-project$";
  position: absolute;
  left: 0;
  color: var(--vp-c-text-3);
}

.code-block.interactive {
  position: relative;
  min-height: 120px;
}

.tab-group {
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  margin: 1rem 0;
  overflow: hidden;
}

.tab-header {
  background-color: var(--vp-c-bg-soft);
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  color: var(--vp-c-text-2);
  border-bottom: 1px solid var(--vp-c-divider);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.tab-dot {
  width: 12px;
  height: 12px;
  background-color: var(--vp-c-brand);
  border-radius: 50%;
  display: inline-block;
}

.compatibility-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.cta {
  text-align: center;
  margin-top: 4rem;
  padding: 2rem;
  background-color: var(--vp-c-bg-soft);
}
.cta a {
  text-decoration: none;
}


@media (max-width: 1024px) {
  .two-column {
    grid-template-columns: 1fr;
    gap: 2rem;
  }

  .content-column {
    padding-top: 0;
  }
}

@media (max-width: 768px) {
  .mise-landing {
    padding: 2rem 1rem;
  }

  .compatibility-grid {
    grid-template-columns: 1fr;
  }

  .button {
    width: 100%;
    max-width: 300px;
  }
}

:root.dark .code-block {
  box-shadow: 0 0 20px rgba(0, 0, 0, 0.2);
}

:root.dark .button.primary {
  box-shadow: 0 4px 12px rgba(var(--vp-c-brand-rgb), 0.4);
}


.code-block::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.code-block::-webkit-scrollbar-track {
  background: var(--vp-c-bg-soft);
  border-radius: 4px;
}

.code-block::-webkit-scrollbar-thumb {
  background: var(--vp-c-brand);
  border-radius: 4px;
}

.code-block::-webkit-scrollbar-thumb:hover {
  background: var(--vp-c-brand-dark);
}

.mise-landing {
  max-width: var(--vp-layout-max-width, 1200px);
  margin: 0 auto;
  padding: 3rem 1.5rem;
}

.code-block pre {
  margin: 0;
  font-family: var(--vp-font-family-mono);
  font-size: 0.9rem;
  line-height: 1.5;
  color: var(--vp-c-text-1);
}

.code-block .section {
  color: #6366f1; /* indigo */
}

.code-block .key {
  color: #2dd4bf; /* teal */
}

.code-block .string {
  color: #22c55e; /* green */
}

.code-block .number {
  color: #f59e0b; /* amber */
}

.code-block .array {
  color: #ec4899; /* pink */
}

.code-block .bracket {
  color: #94a3b8; /* slate */
}

:root.dark .code-block .section {
  color: #818cf8; /* lighter indigo */
}

:root.dark .code-block .key {
  color: #2dd4bf; /* keep teal */
}

:root.dark .code-block .string {
  color: #4ade80; /* lighter green */
}

:root.dark .code-block .number {
  color: #fbbf24; /* lighter amber */
}

:root.dark .code-block .array {
  color: #f472b6; /* lighter pink */
}

:root.dark .code-block .bracket {
  color: #cbd5e1; /* lighter slate */
}
</style>