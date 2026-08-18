/* Aicadia Studio progressive enhancement.
   Every page is server-rendered and works without this file. It adds four
   conveniences and holds no route, state or truth: keyboard focus for the jump
   box, copy buttons, the narrow-screen tree disclosure and a filter over rows
   that are already on the page. */

(function () {
  "use strict";

  document.documentElement.classList.add("js");

  var toast = document.querySelector(".toast");
  var toastTimer = null;

  function announce(message) {
    if (!toast) {
      return;
    }
    toast.textContent = message;
    toast.hidden = false;
    window.clearTimeout(toastTimer);
    toastTimer = window.setTimeout(function () {
      toast.hidden = true;
    }, 2000);
  }

  function isTyping(element) {
    if (!element) {
      return false;
    }
    var name = element.tagName;
    return name === "INPUT" || name === "TEXTAREA" || element.isContentEditable;
  }

  /* `/` focuses the jump box, Escape gives the page back. */
  document.addEventListener("keydown", function (event) {
    if (event.key === "/" && !event.metaKey && !event.ctrlKey && !event.altKey) {
      if (isTyping(event.target)) {
        return;
      }
      var jump = Array.prototype.find.call(
        document.querySelectorAll(".jump input[name='q'], .tree-jump input[name='q']"),
        function (input) {
          return input.offsetParent !== null;
        }
      );
      if (jump) {
        event.preventDefault();
        jump.focus();
        jump.select();
      }
      return;
    }
    if (event.key === "Escape") {
      if (isTyping(event.target)) {
        event.target.blur();
      }
      var openTree = document.getElementById("tree");
      var menuToggle = document.querySelector(".menu-toggle");
      if (openTree && menuToggle && openTree.classList.contains("is-open")) {
        closeTree(true);
      }
    }
  });

  /* Copy buttons write their own `data-copy` text. */
  document.addEventListener("click", function (event) {
    var button = event.target.closest("[data-copy]");
    if (!button) {
      return;
    }
    event.preventDefault();
    var text = button.getAttribute("data-copy") || "";
    if (!navigator.clipboard) {
      announce("Copying needs a secure context");
      return;
    }
    navigator.clipboard.writeText(text).then(
      function () {
        announce("Copied");
      },
      function () {
        announce("Copying was refused");
      }
    );
  });

  /* The narrow-screen tree disclosure. */
  var toggle = document.querySelector(".menu-toggle");
  var tree = document.getElementById("tree");
  var backdrop = document.querySelector(".tree-backdrop");
  var narrow = window.matchMedia("(max-width: 900px)");

  function syncTreeAccess() {
    if (!tree) {
      return;
    }
    tree.inert = narrow.matches && !tree.classList.contains("is-open");
  }

  function closeTree(returnFocus) {
    if (!tree || !toggle) {
      return;
    }
    tree.classList.remove("is-open");
    if (backdrop) {
      backdrop.classList.remove("is-open");
    }
    toggle.setAttribute("aria-expanded", "false");
    toggle.setAttribute("aria-label", "Open section navigation");
    syncTreeAccess();
    if (returnFocus) {
      toggle.focus();
    }
  }

  function openTree() {
    if (!tree || !toggle) {
      return;
    }
    tree.classList.add("is-open");
    if (backdrop) {
      backdrop.classList.add("is-open");
    }
    toggle.setAttribute("aria-expanded", "true");
    toggle.setAttribute("aria-label", "Close section navigation");
    syncTreeAccess();
    var close = tree.querySelector("[data-tree-close]");
    if (close) {
      close.focus();
    }
  }

  if (toggle && tree) {
    toggle.addEventListener("click", function () {
      if (tree.classList.contains("is-open")) {
        closeTree(true);
      } else {
        openTree();
      }
    });
    tree.addEventListener("click", function (event) {
      if (event.target.closest("a")) {
        closeTree(false);
      }
    });
    document.addEventListener("click", function (event) {
      if (event.target.closest("[data-tree-close]")) {
        closeTree(true);
      }
    });
    narrow.addEventListener("change", syncTreeAccess);
    syncTreeAccess();
  }

  /* `[data-filter-rows]` hides non-matching rows of the table it names. */
  function filterRows(input) {
    var table = document.getElementById(input.getAttribute("data-filter-rows"));
    if (!table) {
      return;
    }
    var needle = input.value.trim().toLowerCase();
    var rows = table.querySelectorAll("tbody tr");
    var shown = 0;
    for (var index = 0; index < rows.length; index += 1) {
      var row = rows[index];
      var match = needle === "" || row.textContent.toLowerCase().indexOf(needle) !== -1;
      row.hidden = !match;
      if (match) {
        shown += 1;
      }
    }
    var note = document.querySelector(
      "[data-filter-note='" + input.getAttribute("data-filter-rows") + "']"
    );
    if (note) {
      note.textContent =
        needle === ""
          ? note.getAttribute("data-filter-all") || ""
          : "Showing " + shown + " of " + rows.length + " loaded rows.";
    }
  }

  var filters = document.querySelectorAll("[data-filter-rows]");
  for (var index = 0; index < filters.length; index += 1) {
    filters[index].addEventListener("input", function (event) {
      filterRows(event.target);
    });
  }
})();
