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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="CONTRIBUTING.html"><strong aria-hidden="true">1.</strong> Contributing</a></li><li class="chapter-item expanded "><a href="DEVELOPMENT.html"><strong aria-hidden="true">2.</strong> Development Setup</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Campaign Generation Framework</li><li class="chapter-item expanded "><a href="campaign-framework/index.html"><strong aria-hidden="true">3.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="campaign-framework/01-foundations/index.html"><strong aria-hidden="true">4.</strong> Framework Philosophy</a></li><li class="chapter-item expanded "><a href="campaign-framework/01-foundations/three-board-system.html"><strong aria-hidden="true">5.</strong> The Three-Board System</a></li><li class="chapter-item expanded "><a href="campaign-framework/02-campaign-genesis/index.html"><strong aria-hidden="true">6.</strong> Campaign Genesis Process</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="campaign-framework/02-campaign-genesis/phase-1-spark.html"><strong aria-hidden="true">6.1.</strong> Phase 1: The Spark</a></li><li class="chapter-item expanded "><a href="campaign-framework/02-campaign-genesis/phase-2-session-zero-prep.html"><strong aria-hidden="true">6.2.</strong> Phase 2: Session Zero Prep</a></li><li class="chapter-item expanded "><a href="campaign-framework/02-campaign-genesis/phase-3-session-zero.html"><strong aria-hidden="true">6.3.</strong> Phase 3: Session Zero</a></li><li class="chapter-item expanded "><a href="campaign-framework/02-campaign-genesis/phase-4-integration.html"><strong aria-hidden="true">6.4.</strong> Phase 4: Integration &amp; Launch</a></li></ol></li><li class="chapter-item expanded "><a href="campaign-framework/03-module-creation/index.html"><strong aria-hidden="true">7.</strong> Module Creation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="campaign-framework/03-module-creation/module-creation-process.html"><strong aria-hidden="true">7.1.</strong> Creation Process</a></li><li class="chapter-item expanded "><a href="campaign-framework/03-module-creation/module-types-templates.html"><strong aria-hidden="true">7.2.</strong> Module Types &amp; Templates</a></li><li class="chapter-item expanded "><a href="campaign-framework/03-module-creation/module-running.html"><strong aria-hidden="true">7.3.</strong> Running Your Module</a></li><li class="chapter-item expanded "><a href="campaign-framework/03-module-creation/module-first.html"><strong aria-hidden="true">7.4.</strong> Your First Module</a></li></ol></li><li class="chapter-item expanded "><a href="campaign-framework/04-session-management/index.html"><strong aria-hidden="true">8.</strong> Session Management</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="campaign-framework/04-session-management/session-interlude.html"><strong aria-hidden="true">8.1.</strong> Interlude: Session 1 Play-Through</a></li><li class="chapter-item expanded "><a href="campaign-framework/04-session-management/session-before.html"><strong aria-hidden="true">8.2.</strong> Pre-Session Preparation</a></li><li class="chapter-item expanded "><a href="campaign-framework/04-session-management/session-during.html"><strong aria-hidden="true">8.3.</strong> Running the Session</a></li><li class="chapter-item expanded "><a href="campaign-framework/04-session-management/session-after.html"><strong aria-hidden="true">8.4.</strong> Post-Session &amp; Continuity</a></li></ol></li><li class="chapter-item expanded "><a href="campaign-framework/05-scaling/index.html"><strong aria-hidden="true">9.</strong> Scaling the System</a></li><li class="chapter-item expanded "><a href="campaign-framework/06-templates/index.html"><strong aria-hidden="true">10.</strong> Artifact Templates</a></li></ol>';
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
