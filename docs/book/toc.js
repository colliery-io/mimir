// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Tutorials</li><li class="chapter-item expanded "><a href="tutorials/getting-started.html"><strong aria-hidden="true">1.</strong> Getting Started</a></li><li class="chapter-item expanded "><a href="tutorials/first-campaign.html"><strong aria-hidden="true">2.</strong> Your First Campaign</a></li><li class="chapter-item expanded "><a href="tutorials/setting-up-ollama.html"><strong aria-hidden="true">3.</strong> Setting Up Ollama</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">How-To Guides</li><li class="chapter-item expanded "><a href="how-to/campaign-management.html"><strong aria-hidden="true">4.</strong> Campaign Management</a></li><li class="chapter-item expanded "><a href="how-to/npc-creation.html"><strong aria-hidden="true">5.</strong> NPC Creation &amp; Management</a></li><li class="chapter-item expanded "><a href="how-to/plot-tracking.html"><strong aria-hidden="true">6.</strong> Plot Tracking</a></li><li class="chapter-item expanded "><a href="how-to/database-operations.html"><strong aria-hidden="true">7.</strong> Database Operations</a></li><li class="chapter-item expanded "><a href="how-to/troubleshooting.html"><strong aria-hidden="true">8.</strong> Troubleshooting</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Reference</li><li class="chapter-item expanded "><a href="reference/cli-commands.html"><strong aria-hidden="true">9.</strong> CLI Commands</a></li><li class="chapter-item expanded "><a href="reference/configuration.html"><strong aria-hidden="true">10.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="reference/database-schema.html"><strong aria-hidden="true">11.</strong> Database Schema</a></li><li class="chapter-item expanded "><a href="reference/data-models.html"><strong aria-hidden="true">12.</strong> Data Models</a></li><li class="chapter-item expanded "><a href="reference/api/index.html"><strong aria-hidden="true">13.</strong> API Documentation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="reference/api/core.html"><strong aria-hidden="true">13.1.</strong> Core Types</a></li><li class="chapter-item expanded "><a href="reference/api/database.html"><strong aria-hidden="true">13.2.</strong> Database Layer</a></li><li class="chapter-item expanded "><a href="reference/api/ai.html"><strong aria-hidden="true">13.3.</strong> AI Integration</a></li><li class="chapter-item expanded "><a href="reference/api/agents.html"><strong aria-hidden="true">13.4.</strong> Agent Framework</a></li><li class="chapter-item expanded "><a href="reference/api/tui.html"><strong aria-hidden="true">13.5.</strong> TUI Components</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Explanation</li><li class="chapter-item expanded "><a href="explanation/architecture.html"><strong aria-hidden="true">14.</strong> Architecture Overview</a></li><li class="chapter-item expanded "><a href="explanation/search-strategy.html"><strong aria-hidden="true">15.</strong> Search Strategy</a></li><li class="chapter-item expanded "><a href="explanation/agent-framework.html"><strong aria-hidden="true">16.</strong> Agent Framework</a></li><li class="chapter-item expanded "><a href="explanation/performance.html"><strong aria-hidden="true">17.</strong> Performance Considerations</a></li><li class="chapter-item expanded "><a href="explanation/adr/001-sqlite-vec.html"><strong aria-hidden="true">18.</strong> ADR-001: SQLite with Vector Extensions</a></li><li class="chapter-item expanded "><a href="explanation/adr/002-agent-pattern.html"><strong aria-hidden="true">19.</strong> ADR-002: Agent Communication Patterns</a></li><li class="chapter-item expanded "><a href="explanation/adr/003-tui-framework.html"><strong aria-hidden="true">20.</strong> ADR-003: TUI Framework Selection</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Development</li><li class="chapter-item expanded "><a href="CONTRIBUTING.html"><strong aria-hidden="true">21.</strong> Contributing</a></li><li class="chapter-item expanded "><a href="DEVELOPMENT.html"><strong aria-hidden="true">22.</strong> Development Setup</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
