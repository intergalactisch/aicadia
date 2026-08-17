/* Aicadia Studio progressive enhancement.
   Every page is server-rendered and works without this file. It adds four
   conveniences and holds no route, state or truth: keyboard focus for the jump
   box, copy buttons, the narrow-screen tree disclosure and a filter over rows
   that are already on the page. */

(function () {
  "use strict";

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
      var jump = document.querySelector(".jump input[name='q']");
      if (jump) {
        event.preventDefault();
        jump.focus();
        jump.select();
      }
      return;
    }
    if (event.key === "Escape" && isTyping(event.target)) {
      event.target.blur();
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
  if (toggle && tree) {
    toggle.addEventListener("click", function () {
      var open = tree.classList.toggle("is-open");
      toggle.setAttribute("aria-expanded", open ? "true" : "false");
    });
    tree.addEventListener("click", function (event) {
      if (event.target.closest("a")) {
        tree.classList.remove("is-open");
        toggle.setAttribute("aria-expanded", "false");
      }
    });
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
