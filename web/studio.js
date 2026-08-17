(() => {
  "use strict";

  const GAME_VIEW = ["overview", "models", "tools", "sources", "development"];
  const LIVE_VIEW = ["overview", "entity", "character", "place", "activity", "storage"];
  const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
  const userId = window.__aicadiaUserId;
  let copyReturnFocus = null;

  const state = {
    section: "game",
    view: "overview",
    selectedSource: "game-overview",
    selectedHeading: null,
    selectedModel: "entity",
    selectedTool: null,
    selectedEntityId: null,
    selectedActivityId: null,
    selectedTable: null,
    developmentStatus: null,
    catalog: null,
    world: null,
    storage: null,
    entity: [],
    entityNext: null,
    character: [],
    characterNext: null,
    place: [],
    placeNext: null,
    activity: [],
    activityNext: null,
    entityDetail: null,
    activityDetail: null,
    filter: { entity: "", character: "", place: "", activity: "" },
    error: {},
    readAt: null,
    refreshing: false,
  };

  const all = (selector) => [...document.querySelectorAll(selector)];
  const one = (selector) => document.querySelector(selector);
  const mounts = (name) => all(`[data-mount="${name}"]`);

  function node(tag, className, text) {
    const element = document.createElement(tag);
    if (className) element.className = className;
    if (text !== undefined) element.textContent = text;
    return element;
  }

  function button(text, className, action) {
    const element = node("button", className, text);
    element.type = "button";
    if (action) element.dataset.action = action;
    return element;
  }

  function badge(text, status = "") {
    return node("span", `badge ${status}`.trim(), text);
  }

  function machine(value) {
    const code = node("code", "machine", value ?? "None");
    code.title = value ?? "None";
    return code;
  }

  function empty(message, error = false) {
    return node("div", error ? "error-state" : "empty-state", message);
  }

  function formatDate(value) {
    if (!value) return "Unknown";
    return new Intl.DateTimeFormat(undefined, {
      dateStyle: "medium",
      timeStyle: "medium",
    }).format(new Date(value));
  }

  function titleCase(value) {
    return String(value ?? "")
      .replaceAll("_", " ")
      .replace(/\b\w/g, (letter) => letter.toUpperCase());
  }

  function compactDescription(value) {
    return String(value ?? "").replace(/[#*_`]/g, "").replace(/\s+/g, " ").trim();
  }

  function statusLabel(value) {
    return {
      current: "Current",
      exploratory: "Exploratory",
      experimental: "Experimental",
      evidence: "Evidence",
      planning: "Planning",
      historical: "Historical",
    }[value] ?? titleCase(value);
  }

  function plural(count, singular, pluralValue = `${singular}s`) {
    return `${count} ${count === 1 ? singular : pluralValue}`;
  }

  function currentViews() {
    return state.section === "game" ? GAME_VIEW : LIVE_VIEW;
  }

  function parseLocation() {
    const parameters = new URLSearchParams(location.search);
    state.section = parameters.get("section") === "live" ? "live" : "game";
    const requestedView = parameters.get("view") ?? "overview";
    state.view = currentViews().includes(requestedView) ? requestedView : "overview";
    state.selectedSource = parameters.get("source") ?? state.selectedSource;
    state.selectedHeading = parameters.get("heading");
    state.selectedModel = parameters.get("model") ?? state.selectedModel;
    state.selectedTool = parameters.get("tool");
    state.selectedEntityId = parameters.get("entity");
    state.selectedActivityId = parameters.get("activity");
    state.selectedTable = parameters.get("table");
    state.developmentStatus = parameters.get("status");
  }

  function updateUrl(mode = "push") {
    const url = new URL(location.pathname, location.origin);
    url.searchParams.set("section", state.section);
    url.searchParams.set("view", state.view);
    if (state.section === "game" && state.view === "sources") {
      url.searchParams.set("source", state.selectedSource);
      if (state.selectedHeading) url.searchParams.set("heading", state.selectedHeading);
    }
    if (state.section === "game" && state.view === "models") url.searchParams.set("model", state.selectedModel);
    if (state.section === "game" && state.view === "tools" && state.selectedTool) url.searchParams.set("tool", state.selectedTool);
    if (state.section === "game" && state.view === "development" && state.developmentStatus) url.searchParams.set("status", state.developmentStatus);
    if (state.section === "live" && state.view === "entity" && state.selectedEntityId) url.searchParams.set("entity", state.selectedEntityId);
    if (state.section === "live" && state.view === "activity" && state.selectedActivityId) url.searchParams.set("activity", state.selectedActivityId);
    if (state.section === "live" && state.view === "storage" && state.selectedTable) url.searchParams.set("table", state.selectedTable);
    history[mode === "replace" ? "replaceState" : "pushState"](null, "", url);
  }

  async function getJson(path, contextual = false) {
    const headers = new Headers();
    if (contextual && userId) headers.set("Aicadia-User-Id", userId);
    const response = await fetch(path, { method: "GET", headers });
    const payload = await response.json().catch(() => null);
    if (!response.ok) {
      const message = payload?.error?.message ?? payload?.error ?? `HTTP ${response.status}`;
      throw new Error(message);
    }
    return payload;
  }

  function setConnection(text, status) {
    mounts("connection").forEach((mount) => { mount.textContent = text; });
    all(".connection-state").forEach((element) => { element.dataset.state = status; });
  }

  function toast(message) {
    const element = one("[data-toast]");
    element.textContent = message;
    element.hidden = false;
    window.clearTimeout(toast.timeout);
    toast.timeout = window.setTimeout(() => { element.hidden = true; }, 3600);
  }

  function showCopyFallback(value) {
    const overlay = one("[data-copy-fallback]");
    const field = one("[data-copy-fallback-value]");
    copyReturnFocus = document.activeElement;
    field.textContent = value;
    overlay.hidden = false;
    field.focus();
    const selection = window.getSelection();
    const range = document.createRange();
    range.selectNodeContents(field);
    selection.removeAllRanges();
    selection.addRange(range);
  }

  function closeCopyFallback() {
    one("[data-copy-fallback]").hidden = true;
    if (copyReturnFocus instanceof HTMLElement) copyReturnFocus.focus();
    copyReturnFocus = null;
  }

  async function copyText(value, confirmation = "Reference copied.") {
    let copied = false;
    if (navigator.clipboard?.writeText) {
      try {
        await navigator.clipboard.writeText(value);
        copied = true;
      } catch {}
    }
    if (!copied) {
      const input = document.createElement("textarea");
      input.value = value;
      input.setAttribute("readonly", "");
      input.style.position = "fixed";
      input.style.opacity = "0";
      document.body.append(input);
      input.select();
      try {
        copied = document.execCommand("copy");
      } catch {}
      input.remove();
    }
    if (copied) toast(confirmation);
    else showCopyFallback(value);
  }

  function documents() {
    if (!state.catalog) return [];
    return [...state.catalog.document, state.catalog.agent_contract];
  }

  function selectedSource() {
    return documents().find((source) => source.id === state.selectedSource) ?? documents()[0] ?? null;
  }

  function selectedModel() {
    return state.catalog?.model.find((model) => model.id === state.selectedModel) ?? state.catalog?.model[0] ?? null;
  }

  function selectedTool() {
    return state.catalog?.tool.find((tool) => tool.name === state.selectedTool) ?? state.catalog?.tool[0] ?? null;
  }

  function selectedStorageTable() {
    return state.storage?.table.find((table) => table.name === state.selectedTable) ?? state.storage?.table[0] ?? null;
  }

  function currentReference() {
    const url = location.href;
    if (state.section === "game" && state.view === "sources") {
      const source = selectedSource();
      const heading = source?.heading.find((item) => item.id === state.selectedHeading);
      const locator = `${source?.path ?? "unknown source"}${heading ? `#${heading.id}` : ""}`;
      return `[Aicadia Studio · ${heading?.title ?? source?.title ?? "Source"}](${url}) — source: ${locator}`;
    }
    if (state.section === "game" && state.view === "models") {
      const model = selectedModel();
      const tables = model?.storage_table.length ? `; storage: ${model.storage_table.join(", ")}` : "; no durable table";
      return `[Aicadia Studio · ${model?.title ?? "Model"} model](${url}) — source: ${model?.path ?? "unknown"}${tables}`;
    }
    if (state.section === "game" && state.view === "tools") {
      const tool = selectedTool();
      return `[Aicadia Studio · MCP tool ${tool?.name ?? "unknown"}](${url}) — exact compiled MCP catalog`;
    }
    if (state.section === "live" && state.view === "entity" && state.selectedEntityId) {
      const name = state.entityDetail?.entity?.name ?? "Entity";
      return `[Aicadia Studio · ${name}](${url}) — Entity id: ${state.selectedEntityId}`;
    }
    if (state.section === "live" && state.view === "activity" && state.selectedActivityId) {
      return `[Aicadia Studio · Activity ${state.selectedActivityId}](${url}) — Activity id: ${state.selectedActivityId}`;
    }
    if (state.section === "live" && state.view === "storage") {
      const table = selectedStorageTable();
      return `[Aicadia Studio · PostgreSQL ${table?.name ?? "storage"}](${url}) — schema: public.${table?.name ?? "unknown"}; fingerprint: ${state.storage?.fingerprint ?? "unavailable"}`;
    }
    return `[Aicadia Studio · ${titleCase(state.section)} ${titleCase(state.view)}](${url})`;
  }

  function referenceActions(options = {}) {
    const actions = node("div", "resource-actions");
    const copyReference = button("Copy reference", "button button-secondary", "copy-reference");
    if (options.reference) copyReference.dataset.reference = options.reference;
    actions.append(copyReference, button("Copy link", "button button-ghost", "copy-link"));
    return actions;
  }

  function pageHeader(eyebrow, title, description, actions = null) {
    const header = node("header", "page-header");
    const heading = node("div", "page-heading");
    heading.append(node("div", "eyebrow", eyebrow), node("h1", null, title), node("p", null, description));
    header.append(heading);
    if (actions) {
      actions.classList.add("page-actions");
      header.append(actions);
    }
    return header;
  }

  function resourceHeader(resource, description = null) {
    const header = node("header", "resource-header");
    const status = node("div", "status-line");
    status.append(badge(resource.group ?? "Reference"));
    if (resource.status) status.append(badge(statusLabel(resource.status), resource.status));
    header.append(status, node("h1", null, resource.title));
    if (description) header.append(node("p", null, description));
    if (resource.path) header.append(node("code", "resource-path", resource.path));
    header.append(referenceActions());
    return header;
  }

  function page(body) {
    const wrapper = node("div", "page");
    wrapper.append(body);
    return wrapper;
  }

  function section(title, description = null) {
    const wrapper = node("section", "section");
    const header = node("header", "section-header");
    const copy = node("div");
    copy.append(node("h2", null, title));
    if (description) copy.append(node("p", null, description));
    header.append(copy);
    wrapper.append(header);
    return wrapper;
  }

  function navigationButton(label, view, active, count = null) {
    const item = button("", "side-link", "navigate-view");
    item.dataset.section = state.section;
    item.dataset.view = view;
    item.append(node("span", null, label));
    if (count !== null) item.append(node("small", null, String(count)));
    if (active) {
      item.classList.add("is-active");
      item.setAttribute("aria-current", "page");
    }
    return item;
  }

  function renderNavigation(mount, mobile = false) {
    mount.replaceChildren();
    if (mobile) {
      const sections = node("div", "nav-group");
      sections.append(node("div", "nav-label", "Studio"));
      for (const sectionName of ["game", "live"]) {
        const item = button(titleCase(sectionName), "side-link", "navigate-section");
        item.dataset.section = sectionName;
        if (state.section === sectionName) item.classList.add("is-active");
        sections.append(item);
      }
      mount.append(sections);
    }

    const views = node("div", "nav-group");
    views.append(node("div", "nav-label", state.section === "game" ? "Game" : "Connected World"));
    const labels = state.section === "game"
      ? { overview: "Overview", models: "Models", tools: "MCP tools", sources: "Sources", development: "Development" }
      : { overview: "Overview", entity: "Entities", character: "Characters", place: "Places", activity: "Activity", storage: "Storage" };
    for (const view of currentViews()) {
      views.append(navigationButton(labels[view], view, state.view === view));
    }
    mount.append(views);

    if (!state.catalog) return;
    if (state.section === "game" && state.view === "sources") {
      const grouped = new Map();
      for (const source of documents()) {
        if (!grouped.has(source.group)) grouped.set(source.group, []);
        grouped.get(source.group).push(source);
      }
      for (const [group, sourceList] of grouped) {
        const sourceGroup = node("div", "nav-group");
        sourceGroup.append(node("div", "nav-label", group));
        for (const source of sourceList) {
          const item = button("", "side-link", "open-source");
          item.dataset.source = source.id;
          item.append(node("span", null, source.title), node("small", null, statusLabel(source.status)));
          if (source.id === state.selectedSource) item.classList.add("is-active");
          sourceGroup.append(item);
        }
        mount.append(sourceGroup);
      }
    }
    if (state.section === "game" && state.view === "models") {
      const group = node("div", "nav-group");
      group.append(node("div", "nav-label", "Domain models"));
      for (const model of state.catalog.model) {
        const item = button(model.title, "side-link", "open-model");
        item.dataset.model = model.id;
        if (model.id === state.selectedModel) item.classList.add("is-active");
        group.append(item);
      }
      mount.append(group);
    }
    if (state.section === "game" && state.view === "tools") {
      const group = node("div", "nav-group");
      group.append(node("div", "nav-label", "Compiled catalog"));
      for (const tool of state.catalog.tool) {
        const item = button(tool.name, "side-link", "open-tool");
        item.dataset.tool = tool.name;
        if (tool.name === state.selectedTool) item.classList.add("is-active");
        group.append(item);
      }
      mount.append(group);
    }
    if (state.section === "live" && state.view === "storage" && state.storage) {
      const group = node("div", "nav-group");
      group.append(node("div", "nav-label", "Public schema"));
      for (const table of state.storage.table) {
        const item = button(table.name, "side-link", "open-table");
        item.dataset.table = table.name;
        if (table.name === state.selectedTable) item.classList.add("is-active");
        group.append(item);
      }
      mount.append(group);
    }
  }

  function renderSidebar() {
    mounts("sidebar-context").forEach((mount) => {
      mount.replaceChildren();
      mount.append(node("div", "eyebrow", state.section === "game" ? "Repository truth" : "Read-only connection"));
      mount.append(node("h2", null, state.section === "game" ? "Current Aicadia" : state.world?.name ?? "Aicadia"));
      mount.append(node("p", null, state.section === "game"
        ? "Sources retain their own authority."
        : "One actual local World."));
    });
    mounts("side-nav").forEach((mount) => renderNavigation(mount));
    mounts("mobile-nav").forEach((mount) => renderNavigation(mount, true));
  }

  function renderHeader() {
    all("[data-section-link]").forEach((link) => {
      if (link.dataset.sectionLink === state.section) link.setAttribute("aria-current", "page");
      else link.removeAttribute("aria-current");
    });
    mounts("read-time").forEach((mount) => {
      mount.hidden = !state.readAt;
      mount.dateTime = state.readAt?.toISOString() ?? "";
      mount.textContent = state.readAt ? `Read ${state.readAt.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}` : "";
    });
  }

  function renderStatusCards() {
    const statuses = [
      ["current", "Current contract"],
      ["exploratory", "Open exploration"],
      ["experimental", "Experiments"],
      ["planning", "Planning"],
      ["evidence", "Evidence"],
      ["historical", "Decision history"],
    ];
    const grid = node("div", "status-grid");
    for (const [status, label] of statuses) {
      const count = documents().filter((source) => source.status === status).length;
      const card = button("", "status-card", "open-development-status");
      card.dataset.status = status;
      card.append(node("strong", null, String(count)), node("span", null, label));
      grid.append(card);
    }
    return grid;
  }

  function quickLink(title, description, action, data = {}) {
    const item = button("", "resource-link", action);
    for (const [key, value] of Object.entries(data)) item.dataset[key] = value;
    const copy = node("div");
    copy.append(node("strong", null, title), node("p", null, description));
    item.append(copy, node("code", null, "Open"));
    return item;
  }

  function renderGameOverview() {
    const wrapper = page(node("div"));
    wrapper.firstChild.append(
      pageHeader(
        "Game",
        "The current game, without a second truth",
        "Move from accepted rules to models, exact MCP tools and active development state while every fact remains owned by its original source.",
        referenceActions(),
      ),
    );
    const body = node("div", "page-body");
    body.append(renderStatusCards());
    const start = section("Start from the question", "Each route opens the exact current projection and leaves a reference in the URL.");
    const list = node("div", "resource-list");
    list.append(
      quickLink("Understand the domain model", "Meaning beside current PostgreSQL fields and relationships.", "open-model", { model: "entity" }),
      quickLink("Inspect the MCP surface", `${state.catalog.tool.length} exact tools from the compiled router.`, "open-tool", { tool: state.catalog.tool[0]?.name }),
      quickLink("Read the owning sources", `${documents().length} selected authorities and development records.`, "open-source", { source: "game-overview" }),
      quickLink("See unfinished work", "Exploration, experiments, planning, evidence and history remain visibly distinct.", "open-development-status", { status: "exploratory" }),
    );
    start.append(list);
    body.append(start);
    const boundary = section("Current boundary");
    boundary.append(empty(state.catalog.world_support));
    body.append(boundary);
    wrapper.firstChild.append(body);
    return wrapper;
  }

  function applyHeadingIds(container, source) {
    const headings = [...container.querySelectorAll("h1, h2, h3, h4, h5, h6")];
    headings.forEach((heading, index) => {
      const reference = source.heading[index];
      if (reference) heading.id = reference.id;
    });
  }

  function renderOutline(source) {
    const panel = node("aside", "context-panel");
    panel.append(node("h2", null, "On this page"));
    const list = node("nav", "outline-list");
    list.setAttribute("aria-label", "Source outline");
    for (const heading of source.heading) {
      const item = button(heading.title, "outline-link", "open-heading");
      item.dataset.heading = heading.id;
      item.dataset.level = String(heading.level);
      if (state.selectedHeading === heading.id) item.classList.add("is-active");
      list.append(item);
    }
    panel.append(list);
    return panel;
  }

  function renderSources() {
    const source = selectedSource();
    if (!source) return page(empty("No source is available.", true));
    const wrapper = page(node("div"));
    const grid = node("div", "page-grid");
    const content = node("div", "content-column");
    content.append(resourceHeader(source));
    const prose = node("article", "prose source-prose");
    prose.innerHTML = source.html;
    applyHeadingIds(prose, source);
    content.append(prose);
    const context = node("div", "context-column");
    context.append(renderOutline(source));
    const provenance = node("section", "context-panel");
    provenance.append(node("h2", null, "Provenance"), node("p", null, "Studio renders this file directly. Changes remain owned by the repository source."));
    provenance.append(machine(source.path));
    context.append(provenance);
    grid.append(content, context);
    wrapper.firstChild.append(grid);
    window.requestAnimationFrame(() => {
      if (state.selectedHeading) one(`#${CSS.escape(state.selectedHeading)}`)?.scrollIntoView({ block: "start" });
    });
    return wrapper;
  }

  function relationsForTables(tableNames) {
    return state.storage?.relation.filter((relation) => tableNames.includes(relation.table) || tableNames.includes(relation.referenced_table)) ?? [];
  }

  function relationText(relation) {
    return `${relation.table}(${relation.columns.join(", ")}) → ${relation.referenced_table}(${relation.referenced_columns.join(", ")})`;
  }

  function renderRelationList(relations) {
    if (!relations.length) return empty("No PostgreSQL foreign-key relation is attached to this selection.");
    const list = node("div", "relation-list");
    for (const relation of relations) {
      const row = node("div", "relation-row");
      row.append(node("code", null, relationText(relation)));
      const copy = button("Copy", "button button-ghost button-small", "copy-value");
      copy.dataset.value = `PostgreSQL relation: ${relationText(relation)} (${relation.name})`;
      row.append(copy);
      list.append(row);
    }
    return list;
  }

  function renderColumnTable(table, compact = false) {
    const scroll = node("div", "table-scroll");
    const tableElement = node("table", "data-table");
    const head = node("thead");
    const headerRow = node("tr");
    for (const label of ["Column", "Type", "Null", ...(compact ? [] : ["Default", "Reference"])]) headerRow.append(node("th", null, label));
    head.append(headerRow);
    const body = node("tbody");
    for (const column of table.column) {
      const row = node("tr");
      row.append(node("td", null, column.name), node("td", null, column.data_type), node("td", null, column.nullable ? "Yes" : "No"));
      if (!compact) {
        const defaultCell = node("td");
        defaultCell.append(machine(column.default_value ?? "—"));
        const referenceCell = node("td");
        const copy = button("Copy", "button button-ghost button-small", "copy-value");
        copy.dataset.value = `PostgreSQL column: public.${table.name}.${column.name} (${column.data_type}${column.nullable ? ", nullable" : ", required"})`;
        referenceCell.append(copy);
        row.append(defaultCell, referenceCell);
      }
      body.append(row);
    }
    tableElement.append(head, body);
    scroll.append(tableElement);
    return scroll;
  }

  function renderModels() {
    const model = selectedModel();
    if (!model) return page(empty("No model is available.", true));
    const wrapper = page(node("div"));
    const grid = node("div", "page-grid model-grid");
    const content = node("div", "content-column");
    content.append(resourceHeader(model, "Semantic meaning from the owning domain contract, realized structure from the connected PostgreSQL schema."));
    const meaning = section("Meaning", "Rendered directly from the current domain contract.");
    const prose = node("article", "prose");
    prose.innerHTML = model.html;
    applyHeadingIds(prose, model);
    meaning.append(prose);
    content.append(meaning);

    const realized = section("Realized storage", model.storage_table.length
      ? "Columns and constraints below are introspected from the connected database."
      : "This behavior seam deliberately has no durable table of its own.");
    const storage = node("div", "model-storage");
    for (const tableName of model.storage_table) {
      const table = state.storage?.table.find((candidate) => candidate.name === tableName);
      if (!table) {
        storage.append(empty(`Storage table ${tableName} is not present in the connected schema.`, true));
        continue;
      }
      const summary = node("section", "storage-table-summary");
      const heading = node("header", "storage-table-heading");
      heading.append(node("h3", null, table.name));
      const open = button("Open full table", "table-link", "open-table");
      open.dataset.table = table.name;
      heading.append(open);
      summary.append(heading, renderColumnTable(table, true));
      storage.append(summary);
    }
    realized.append(storage);
    content.append(realized);

    const relationships = section("Relationships", "Exact ordered foreign keys touching this model's storage tables.");
    relationships.append(renderRelationList(relationsForTables(model.storage_table)));
    content.append(relationships);

    const context = node("div", "context-column");
    const sourcePanel = node("section", "context-panel");
    sourcePanel.append(node("h2", null, "Owning source"), machine(model.path));
    const openSource = button("Open source", "button button-secondary", "open-source");
    openSource.dataset.source = "domain";
    openSource.dataset.heading = model.heading[0]?.id ?? "";
    const sourceActions = node("div", "inline-actions");
    sourceActions.append(openSource);
    sourcePanel.append(sourceActions);
    context.append(sourcePanel);
    const tablesPanel = node("section", "context-panel");
    tablesPanel.append(node("h2", null, "Storage tables"));
    if (!model.storage_table.length) tablesPanel.append(node("p", null, "No durable table. The World remains a behavior seam."));
    for (const tableName of model.storage_table) {
      const open = button(tableName, "side-link", "open-table");
      open.dataset.table = tableName;
      tablesPanel.append(open);
    }
    context.append(tablesPanel);
    grid.append(content, context);
    wrapper.firstChild.append(grid);
    return wrapper;
  }

  function renderTools() {
    const tool = selectedTool();
    if (!tool) return page(empty("No MCP tool is available.", true));
    const readOnly = Boolean(tool.annotations?.readOnlyHint);
    const resource = {
      title: tool.title ?? titleCase(tool.name),
      group: "Exact MCP tool",
      status: readOnly ? "read-only" : "mutation",
      path: `compiled MCP router · ${tool.name}`,
    };
    const wrapper = page(node("div"));
    const grid = node("div", "page-grid");
    const content = node("div", "content-column");
    content.append(resourceHeader(resource));
    const meaning = section("Agent-facing description");
    meaning.append(node("p", null, tool.description));
    content.append(meaning);
    const schema = section("Input schema", "This JSON Schema is taken from the exact compiled MCP catalog.");
    const pre = node("pre");
    const code = node("code", null, JSON.stringify(tool.inputSchema, null, 2));
    pre.append(code);
    schema.append(pre);
    content.append(schema);
    const context = node("div", "context-column");
    const annotations = node("section", "context-panel");
    annotations.append(node("h2", null, "Annotations"));
    const list = node("dl", "schema-meta");
    for (const [name, value] of Object.entries(tool.annotations ?? {})) {
      const item = node("div");
      item.append(node("dt", null, titleCase(name)), node("dd", null, String(value)));
      list.append(item);
    }
    annotations.append(list);
    context.append(annotations);
    grid.append(content, context);
    wrapper.firstChild.append(grid);
    return wrapper;
  }

  function sourceList(sourceList) {
    const list = node("div", "resource-list");
    for (const source of sourceList) {
      const item = button("", "resource-link", "open-source");
      item.dataset.source = source.id;
      const copy = node("div");
      copy.append(node("strong", null, source.title), node("p", null, `${source.group} · ${statusLabel(source.status)}`));
      item.append(copy, node("code", null, source.path));
      list.append(item);
    }
    return list;
  }

  function renderDevelopment() {
    const wrapper = page(node("div"));
    const actions = referenceActions();
    if (state.developmentStatus) {
      const clear = button("Show all states", "button button-secondary", "clear-development-status");
      actions.prepend(clear);
    }
    wrapper.firstChild.append(pageHeader(
      "Development",
      state.developmentStatus ? statusLabel(state.developmentStatus) : "Current state and unfinished work",
      "Accepted truth, exploration, experiments, planning, evidence and history stay separate and directly traceable.",
      actions,
    ));
    const body = node("div", "page-body");
    const statuses = state.developmentStatus
      ? [state.developmentStatus]
      : ["current", "exploratory", "experimental", "planning", "evidence", "historical"];
    for (const status of statuses) {
      const matching = documents().filter((source) => source.status === status);
      if (!matching.length) continue;
      const group = section(statusLabel(status), plural(matching.length, "source"));
      group.append(sourceList(matching));
      body.append(group);
    }
    wrapper.firstChild.append(body);
    return wrapper;
  }

  function visibleCount(items, next) {
    return `${items.length}${next ? "+" : ""}`;
  }

  function renderLiveOverview() {
    const wrapper = page(node("div"));
    wrapper.firstChild.append(pageHeader(
      "Live",
      state.world?.name ?? "Aicadia",
      "A bounded, read-only view of the connected local World and the PostgreSQL structure that currently realizes it.",
      referenceActions(),
    ));
    const body = node("div", "page-body");
    const grid = node("div", "status-grid");
    for (const [kind, label, items, next] of [
      ["entity", "Entities visible", state.entity, state.entityNext],
      ["character", "Characters visible", state.character, state.characterNext],
      ["place", "Places visible", state.place, state.placeNext],
      ["activity", "Activity visible", state.activity, state.activityNext],
      ["storage", "Schema tables", state.storage?.table ?? [], null],
    ]) {
      const card = button("", "status-card", "navigate-view");
      card.dataset.section = "live";
      card.dataset.view = kind;
      card.append(node("strong", null, visibleCount(items, next)), node("span", null, label));
      grid.append(card);
    }
    body.append(grid);
    const paths = section("Follow the World", "Role rows and history always link back to their stable Entity or Activity identity.");
    const list = node("div", "resource-list");
    list.append(
      quickLink("Browse durable Entities", "Open current Properties, Traits, roles and Place.", "navigate-view", { section: "live", view: "entity" }),
      quickLink("Inspect accepted Activity", userId ? "Open durable history and its involved Entities." : "Personal Activity needs the development User context.", "navigate-view", { section: "live", view: "activity" }),
      quickLink("Understand storage", `${state.storage?.relation.length ?? 0} exact foreign-key relationships in the current schema.`, "open-table", { table: state.storage?.table[0]?.name }),
    );
    paths.append(list);
    body.append(paths);
    wrapper.firstChild.append(body);
    return wrapper;
  }

  function filterToolbar(kind, options = {}) {
    const toolbar = node("div", "data-toolbar");
    const field = node("div", "field");
    const id = `filter-${kind}`;
    const label = node("label", null, `Filter loaded ${options.label ?? `${kind} records`}`);
    label.htmlFor = id;
    const input = node("input");
    input.id = id;
    input.name = `filter_${kind}`;
    input.type = "search";
    input.value = state.filter[kind] ?? "";
    input.placeholder = "Filter loaded results";
    input.dataset.filter = kind;
    field.append(label, input);
    toolbar.append(field, node("span", "loaded-note", "Filtering applies only to records loaded below."));
    return toolbar;
  }

  function directEntityToolbar() {
    const wrapper = node("div", "filter-row direct-id");
    const field = node("div", "field");
    const label = node("label", null, "Open an exact Entity id");
    label.htmlFor = "direct-entity-id";
    const input = node("input");
    input.id = "direct-entity-id";
    input.name = "entity_id";
    input.type = "text";
    input.placeholder = "00000000-0000-0000-0000-000000000000";
    input.autocomplete = "off";
    field.append(label, input);
    wrapper.append(field, button("Open", "button button-secondary", "open-direct-entity"));
    return wrapper;
  }

  function filtered(kind, items, values) {
    const query = (state.filter[kind] ?? "").trim().toLowerCase();
    if (!query) return items;
    return items.filter((item) => values(item).some((value) => String(value ?? "").toLowerCase().includes(query)));
  }

  function loadMoreButton(kind, next) {
    if (!next) return null;
    const more = button(`Load older ${titleCase(kind)}`, "button button-secondary load-more", "load-more");
    more.dataset.kind = kind;
    return more;
  }

  function listHeader(kind, title, description) {
    const actions = referenceActions();
    if (kind === "entity") actions.prepend(directEntityToolbar());
    return pageHeader("Live", title, description, actions);
  }

  function renderEntityList() {
    if (state.error.entity) return page(empty(state.error.entity, true));
    const wrapper = page(node("div"));
    wrapper.firstChild.append(listHeader("entity", "Entities", "Every durable World subject, independent of its current role."));
    const body = node("div", "page-body");
    body.append(filterToolbar("entity", { label: "Entities" }));
    const items = filtered("entity", state.entity, (entity) => [entity.id, entity.name, entity.description]);
    if (!items.length) body.append(empty(state.entity.length ? "No loaded Entity matches this filter." : "No Entity records exist yet."));
    else {
      const scroll = node("div", "table-scroll");
      const table = node("table", "data-table");
      const head = node("thead");
      const row = node("tr");
      for (const label of ["Entity", "Stable id", "Description"]) row.append(node("th", null, label));
      head.append(row);
      const tbody = node("tbody");
      for (const entity of items) {
        const item = node("tr");
        const name = node("td");
        const open = button(entity.name, "record-link", "open-entity");
        open.dataset.entity = entity.id;
        name.append(open);
        const id = node("td"); id.append(machine(entity.id));
        item.append(name, id, node("td", "muted", entity.description));
        tbody.append(item);
      }
      table.append(head, tbody); scroll.append(table); body.append(scroll);
    }
    const more = loadMoreButton("entity", state.entityNext); if (more) body.append(more);
    wrapper.firstChild.append(body);
    return wrapper;
  }

  function renderCharacterList() {
    if (state.error.character) return page(empty(state.error.character, true));
    const wrapper = page(node("div"));
    wrapper.firstChild.append(listHeader("character", "Characters", "User-owned roles that retain their Entity identity and current Place."));
    const body = node("div", "page-body");
    body.append(filterToolbar("character", { label: "Characters" }));
    const items = filtered("character", state.character, (item) => [item.id, item.name, item.owner_user_id, item.current_place_name]);
    if (!items.length) body.append(empty(state.character.length ? "No loaded Character matches this filter." : "No Character roles exist yet."));
    else {
      const scroll = node("div", "table-scroll"); const table = node("table", "data-table");
      const head = node("thead"); const row = node("tr");
      for (const label of ["Character", "Owner User", "Current Place", "Introduced"]) row.append(node("th", null, label)); head.append(row);
      const bodyRows = node("tbody");
      for (const character of items) {
        const item = node("tr");
        const name = node("td"); const open = button(character.name, "record-link", "open-entity"); open.dataset.entity = character.id; name.append(open);
        const owner = node("td"); owner.append(machine(character.owner_user_id));
        const place = node("td");
        if (character.current_place_entity_id) {
          const openPlace = button(character.current_place_name ?? "Place", "record-link", "open-entity"); openPlace.dataset.entity = character.current_place_entity_id; place.append(openPlace);
        } else place.textContent = "Not entered";
        item.append(name, owner, place, node("td", "muted", formatDate(character.introduced_at))); bodyRows.append(item);
      }
      table.append(head, bodyRows); scroll.append(table); body.append(scroll);
    }
    const more = loadMoreButton("character", state.characterNext); if (more) body.append(more);
    wrapper.firstChild.append(body); return wrapper;
  }

  function renderPlaceList() {
    if (state.error.place) return page(empty(state.error.place, true));
    const wrapper = page(node("div"));
    wrapper.firstChild.append(listHeader("place", "Places", "World locations using stable Entity identity."));
    const body = node("div", "page-body"); body.append(filterToolbar("place", { label: "Places" }));
    const items = filtered("place", state.place, (item) => [item.id, item.name, item.description]);
    if (!items.length) body.append(empty(state.place.length ? "No loaded Place matches this filter." : "No Place roles exist yet."));
    else {
      const scroll = node("div", "table-scroll"); const table = node("table", "data-table");
      const head = node("thead"); const row = node("tr");
      for (const label of ["Place", "Role", "Latest Activity", "Description"]) row.append(node("th", null, label)); head.append(row);
      const rows = node("tbody");
      for (const place of items) {
        const item = node("tr"); const name = node("td"); const open = button(place.name, "record-link", "open-entity"); open.dataset.entity = place.id; name.append(open);
        const activity = node("td"); const openActivity = button("Open Activity", "record-link", "open-activity"); openActivity.dataset.activity = place.latest_activity_id; activity.append(openActivity);
        item.append(name, node("td", null, place.is_entry ? "Entry Place" : "Place"), activity, node("td", "muted", place.description)); rows.append(item);
      }
      table.append(head, rows); scroll.append(table); body.append(scroll);
    }
    const more = loadMoreButton("place", state.placeNext); if (more) body.append(more);
    wrapper.firstChild.append(body); return wrapper;
  }

  function renderActivityList() {
    if (!userId) return page(empty("Personal Activity needs the development User context supplied by cargo dev."));
    if (state.error.activity) return page(empty(state.error.activity, true));
    const wrapper = page(node("div"));
    wrapper.firstChild.append(listHeader("activity", "Activity", "Durable accepted history involving the local Character."));
    const body = node("div", "page-body"); body.append(filterToolbar("activity", { label: "Activity" }));
    const items = filtered("activity", state.activity, (item) => [item.id, item.operation, item.prose, item.involved_entity?.map((reference) => reference.entity.name).join(" ")]);
    if (!items.length) body.append(empty(state.activity.length ? "No loaded Activity matches this filter." : "No accepted Activity involves this Character yet."));
    else {
      const list = node("div", "activity-list");
      for (const activity of items) {
        const card = button("", "activity-card", "open-activity"); card.dataset.activity = activity.id;
        const header = node("header"); header.append(node("h3", null, activity.operation), node("time", null, formatDate(activity.occurred_at)));
        card.append(header); if (activity.prose) card.append(node("p", null, activity.prose));
        const roles = activity.involved_entity?.map((reference) => `${reference.role}: ${reference.entity.name}`).join(" · ");
        if (roles) card.append(node("small", null, roles)); list.append(card);
      }
      body.append(list);
    }
    const more = loadMoreButton("activity", state.activityNext); if (more) body.append(more);
    wrapper.firstChild.append(body); return wrapper;
  }

  function metadata(facts) {
    const list = node("dl", "detail-meta");
    for (const [label, value, action] of facts) {
      const item = node("div"); item.append(node("dt", null, label)); const detail = node("dd");
      if (action) detail.append(action); else detail.textContent = value; item.append(detail); list.append(item);
    }
    return list;
  }

  function renderEntityDetail() {
    if (state.entityDetail?.loading) return page(node("div", "loading-state", "Reading current Entity state."));
    if (state.entityDetail?.error) return page(empty(state.entityDetail.error, true));
    const detail = state.entityDetail;
    if (!detail?.entity) return renderEntityList();
    const entity = detail.entity;
    const roles = [entity.is_character && "Character", entity.is_place && "Place", "Entity"].filter(Boolean);
    const wrapper = page(node("div"));
    const actions = referenceActions(); actions.prepend(button("All Entities", "button button-secondary", "clear-entity"));
    wrapper.firstChild.append(pageHeader(roles.join(" · "), entity.name, entity.description, actions));
    const placeAction = entity.place_entity_id ? button(entity.place_name ?? "Place", "record-link", "open-entity") : null;
    if (placeAction) placeAction.dataset.entity = entity.place_entity_id;
    wrapper.firstChild.append(metadata([
      ["Entity id", entity.id],
      ["Introduced", formatDate(entity.introduced_at)],
      ["Current Place", entity.place_name ?? "No explicit Place", placeAction],
    ]));
    const properties = section(`Properties · ${detail.property.length}${detail.property_truncated ? "+" : ""}`, "Current typed values and the Activity that established each pointer.");
    if (!detail.property.length) properties.append(empty("No current Properties."));
    else {
      const list = node("div", "state-list");
      for (const property of detail.property) {
        const value = property.value_type === "integer" ? property.integer_value : property.text_value;
        const row = node("div", "state-row"); const copy = node("div"); copy.append(node("code", null, property.key), node("p", null, String(value)));
        const actionsRow = node("div", "inline-actions");
        const activity = button("Activity", "button button-ghost button-small", "open-activity"); activity.dataset.activity = property.current_activity_id;
        const reference = button("Copy", "button button-ghost button-small", "copy-value"); reference.dataset.value = `Aicadia Entity ${entity.id} Property ${property.key} = ${value} (${property.value_type}); current Activity ${property.current_activity_id}`;
        actionsRow.append(activity, reference); row.append(copy, actionsRow); list.append(row);
      }
      properties.append(list);
    }
    wrapper.firstChild.append(properties);
    const traits = section(`Traits · ${detail.trait.length}${detail.trait_truncated ? "+" : ""}`, "Current statements retain stable Trait identity and Activity provenance.");
    if (!detail.trait.length) traits.append(empty("No current Traits."));
    else {
      const list = node("div", "state-list");
      for (const trait of detail.trait) {
        const row = node("div", "state-row"); const copy = node("div"); copy.append(machine(trait.id), node("p", null, trait.statement));
        const actionsRow = node("div", "inline-actions");
        const activity = button("Activity", "button button-ghost button-small", "open-activity"); activity.dataset.activity = trait.current_activity_id;
        const reference = button("Copy", "button button-ghost button-small", "copy-value"); reference.dataset.value = `Aicadia Entity ${entity.id} Trait ${trait.id}: ${trait.statement}; current Activity ${trait.current_activity_id}`;
        actionsRow.append(activity, reference); row.append(copy, actionsRow); list.append(row);
      }
      traits.append(list);
    }
    wrapper.firstChild.append(traits); return wrapper;
  }

  function renderActivityDetail() {
    if (state.activityDetail?.loading) return page(node("div", "loading-state", "Reading Activity."));
    if (state.activityDetail?.error) return page(empty(state.activityDetail.error, true));
    const detail = state.activityDetail;
    if (!detail?.activity) return renderActivityList();
    const activity = detail.activity;
    const wrapper = page(node("div")); const actions = referenceActions(); actions.prepend(button("All Activity", "button button-secondary", "clear-activity"));
    wrapper.firstChild.append(pageHeader("Accepted Activity", titleCase(activity.operation), activity.prose ?? "This accepted Activity has no prose.", actions));
    wrapper.firstChild.append(metadata([
      ["Activity id", activity.id],
      ["Occurred", formatDate(activity.occurred_at)],
      ["Consequence", activity.action_consequence ?? "Not an Action consequence"],
    ]));
    const involved = section(`Involved Entities · ${detail.involved_entity.length}${detail.involved_entity_truncated ? "+" : ""}`, "Stable roles stored with this accepted Activity.");
    if (!detail.involved_entity.length) involved.append(empty("No involved Entity roles were stored."));
    else {
      const list = node("div", "resource-list");
      for (const reference of detail.involved_entity) {
        const item = button("", "resource-link", "open-entity"); item.dataset.entity = reference.entity_id;
        const copy = node("div"); copy.append(node("strong", null, reference.entity_name), node("p", null, titleCase(reference.role)));
        item.append(copy, machine(reference.entity_id)); list.append(item);
      }
      involved.append(list);
    }
    wrapper.firstChild.append(involved); return wrapper;
  }

  function definitionItems(items, referencePrefix) {
    if (!items.length) return empty("None.");
    const list = node("div", "definition-list");
    for (const item of items) {
      const wrapper = node("div", "definition-item"); wrapper.append(node("strong", null, item.name), node("code", null, item.definition));
      const copy = button("Copy reference", "button button-ghost button-small", "copy-value"); copy.dataset.value = `${referencePrefix} ${item.name}: ${item.definition}`; wrapper.append(copy); list.append(wrapper);
    }
    return list;
  }

  function renderStorage() {
    if (state.error.storage) return page(empty(state.error.storage, true));
    if (!state.storage) return page(empty("Storage metadata is unavailable."));
    const table = selectedStorageTable();
    if (!table) return page(empty("No public application tables are present."));
    const wrapper = page(node("div"));
    const actions = referenceActions();
    const download = node("a", "button button-primary", "Download schema snapshot");
    download.href = "/studio/api/live/storage/snapshot";
    download.setAttribute("download", "aicadia-schema-snapshot.json");
    actions.prepend(download);
    wrapper.firstChild.append(pageHeader("Live storage", `public.${table.name}`, "Exact connected PostgreSQL structure. No table rows are included in this view or its downloadable snapshot.", actions));
    const grid = node("div", "page-grid"); const content = node("div", "content-column");
    const columns = section(`Columns · ${table.column.length}`); columns.append(renderColumnTable(table)); content.append(columns);
    const constraints = section(`Constraints · ${table.constraint.length}`); constraints.append(definitionItems(table.constraint, `PostgreSQL constraint public.${table.name}`)); content.append(constraints);
    const relations = state.storage.relation.filter((relation) => relation.table === table.name || relation.referenced_table === table.name);
    const relationSection = section(`Relationships · ${relations.length}`, "Exact ordered foreign keys entering or leaving this table."); relationSection.append(renderRelationList(relations)); content.append(relationSection);
    const indexes = section(`Indexes · ${table.index.length}`); indexes.append(definitionItems(table.index, `PostgreSQL index public.${table.name}`)); content.append(indexes);
    const context = node("div", "context-column"); const snapshot = node("section", "context-panel"); snapshot.append(node("h2", null, "Schema snapshot"));
    const meta = node("dl", "schema-meta");
    for (const [label, value] of [
      ["Captured", formatDate(state.storage.captured_at)],
      ["Latest migration", state.storage.latest_migration ?? "None"],
      ["Tables", state.storage.table.length],
      ["Foreign keys", state.storage.relation.length],
      ["Fingerprint", state.storage.fingerprint],
    ]) {
      const item = node("div"); item.append(node("dt", null, label), node("dd", null, String(value))); meta.append(item);
    }
    snapshot.append(meta); context.append(snapshot); grid.append(content, context);
    const body = node("div", "page-body"); body.append(grid); wrapper.firstChild.append(body);
    return wrapper;
  }

  function renderCurrentPage() {
    if (!state.catalog) return page(empty(state.error.catalog ?? "The Studio catalog is unavailable.", true));
    if (state.section === "game") {
      return { overview: renderGameOverview, models: renderModels, tools: renderTools, sources: renderSources, development: renderDevelopment }[state.view]();
    }
    if (state.view === "entity" && state.selectedEntityId) return renderEntityDetail();
    if (state.view === "activity" && state.selectedActivityId) return renderActivityDetail();
    return { overview: renderLiveOverview, entity: renderEntityList, character: renderCharacterList, place: renderPlaceList, activity: renderActivityList, storage: renderStorage }[state.view]();
  }

  function renderApp() {
    if (state.catalog) {
      if (!state.catalog.model.some((model) => model.id === state.selectedModel)) state.selectedModel = state.catalog.model[0]?.id ?? "entity";
      if (!state.selectedTool || !state.catalog.tool.some((tool) => tool.name === state.selectedTool)) state.selectedTool = state.catalog.tool[0]?.name ?? null;
      if (!documents().some((source) => source.id === state.selectedSource)) state.selectedSource = documents()[0]?.id ?? "game-overview";
    }
    if (state.storage && !state.storage.table.some((table) => table.name === state.selectedTable)) state.selectedTable = state.storage.table[0]?.name ?? null;
    renderHeader(); renderSidebar();
    mounts("page").forEach((mount) => mount.replaceChildren(renderCurrentPage()));
  }

  function focusContent() {
    one("#studio-content")?.focus({ preventScroll: true });
    window.scrollTo({ top: 0, behavior: "auto" });
  }

  function closeMobileNavigation() {
    const panel = one("#mobile-navigation"); const toggle = one('[data-action="toggle-mobile-nav"]');
    panel.hidden = true; toggle.setAttribute("aria-expanded", "false");
  }

  function navigate(section, view, mode = "push") {
    state.section = section; state.view = (section === "game" ? GAME_VIEW : LIVE_VIEW).includes(view) ? view : "overview";
    if (state.view !== "entity") { state.selectedEntityId = null; state.entityDetail = null; }
    if (state.view !== "activity") { state.selectedActivityId = null; state.activityDetail = null; }
    updateUrl(mode); closeMobileNavigation(); renderApp(); focusContent();
  }

  function openSource(sourceId, heading = null, mode = "push") {
    state.section = "game"; state.view = "sources"; state.selectedSource = sourceId; state.selectedHeading = heading || null;
    updateUrl(mode); closeMobileNavigation(); renderApp(); focusContent();
  }

  function openModel(modelId, mode = "push") {
    state.section = "game"; state.view = "models"; state.selectedModel = modelId;
    updateUrl(mode); closeMobileNavigation(); renderApp(); focusContent();
  }

  function openTool(toolName, mode = "push") {
    state.section = "game"; state.view = "tools"; state.selectedTool = toolName;
    updateUrl(mode); closeMobileNavigation(); renderApp(); focusContent();
  }

  function openTable(tableName, mode = "push") {
    state.section = "live"; state.view = "storage"; state.selectedTable = tableName;
    updateUrl(mode); closeMobileNavigation(); renderApp(); focusContent();
  }

  async function openEntity(entityId, mode = "push") {
    state.section = "live"; state.view = "entity"; state.selectedEntityId = entityId; state.entityDetail = { loading: true };
    updateUrl(mode); closeMobileNavigation(); renderApp(); focusContent();
    try { state.entityDetail = await getJson(`/studio/api/live/entity/${encodeURIComponent(entityId)}`); }
    catch (error) { state.entityDetail = { error: error.message }; }
    renderApp();
  }

  async function openActivity(activityId, mode = "push") {
    state.section = "live"; state.view = "activity"; state.selectedActivityId = activityId; state.activityDetail = { loading: true };
    updateUrl(mode); closeMobileNavigation(); renderApp(); focusContent();
    try { state.activityDetail = await getJson(`/studio/api/live/activity/${encodeURIComponent(activityId)}`); }
    catch (error) { state.activityDetail = { error: error.message }; }
    renderApp();
  }

  function linkedSourcePath(referencePath, href) {
    const hrefPath = href.split("#", 1)[0];
    if (!hrefPath || hrefPath.includes(":") || hrefPath.startsWith("/")) return null;
    const parts = [...referencePath.split("/").slice(0, -1), ...hrefPath.split("/")]; const normalized = [];
    for (const part of parts) { if (!part || part === ".") continue; if (part === "..") normalized.pop(); else normalized.push(part); }
    return normalized.join("/");
  }

  function openSourceLink(link) {
    const href = link.getAttribute("href") ?? ""; const source = selectedSource();
    if (/^https?:/i.test(href)) return false;
    const [pathPart, fragment] = href.split("#", 2);
    if (!pathPart) { openSource(source.id, fragment || null); return true; }
    const path = source?.path ? linkedSourcePath(source.path, href) : null;
    const target = path && documents().find((candidate) => candidate.path === path);
    if (target) { openSource(target.id, fragment || null); return true; }
    toast(`Linked source is outside the current Studio catalog: ${href}`); return true;
  }

  async function loadMore(kind) {
    const configuration = {
      entity: [state.entityNext && `/api/entity?limit=24&cursor=${encodeURIComponent(state.entityNext)}`, "entity", "next"],
      character: [state.characterNext && `/studio/api/live/character?limit=24&before=${encodeURIComponent(state.characterNext)}`, "character", "next_cursor"],
      place: [state.placeNext && `/studio/api/live/place?limit=24&before=${encodeURIComponent(state.placeNext)}`, "place", "next_cursor"],
      activity: [state.activityNext && `/api/activity?limit=24&cursor=${encodeURIComponent(state.activityNext)}`, "activity", "next"],
    }[kind];
    if (!configuration?.[0]) return;
    try {
      const response = await getJson(configuration[0], kind === "activity"); state[kind].push(...response[configuration[1]]); state[`${kind}Next`] = response[configuration[2]]; renderApp();
    } catch (error) { toast(error.message); }
  }

  async function loadInitial() {
    setConnection("Reading", "loading");
    const requests = [
      ["catalog", getJson("/studio/api/catalog")], ["world", getJson("/api/world")], ["entity", getJson("/api/entity?limit=24")],
      ["character", getJson("/studio/api/live/character?limit=24")], ["place", getJson("/studio/api/live/place?limit=24")], ["storage", getJson("/studio/api/live/storage")],
    ];
    if (userId) requests.push(["activity", getJson("/api/activity?limit=24", true)]);
    const results = await Promise.allSettled(requests.map(([, request]) => request));
    results.forEach((result, index) => {
      const key = requests[index][0];
      if (result.status === "rejected") { state.error[key] = result.reason?.message ?? `${titleCase(key)} is unavailable.`; return; }
      const value = result.value;
      if (key === "catalog") state.catalog = value;
      if (key === "world") state.world = value;
      if (key === "entity") { state.entity = value.entity; state.entityNext = value.next; }
      if (key === "character") { state.character = value.character; state.characterNext = value.next_cursor; }
      if (key === "place") { state.place = value.place; state.placeNext = value.next_cursor; }
      if (key === "activity") { state.activity = value.activity; state.activityNext = value.next; }
      if (key === "storage") state.storage = value;
    });
    state.readAt = new Date(); setConnection(state.world ? "Connected" : "Partial", state.world ? "connected" : "error");
    renderApp();
    if (state.selectedEntityId) await openEntity(state.selectedEntityId, "replace");
    if (state.selectedActivityId) await openActivity(state.selectedActivityId, "replace");
    const loading = one("[data-loading]"); loading.classList.add("is-done"); window.setTimeout(() => { loading.hidden = true; }, 180);
  }

  async function refresh() {
    if (state.refreshing) return; state.refreshing = true; all('[data-action="refresh"]').forEach((item) => { item.disabled = true; });
    state.error = {}; state.entity = []; state.character = []; state.place = []; state.activity = [];
    await loadInitial(); state.refreshing = false; all('[data-action="refresh"]').forEach((item) => { item.disabled = false; }); toast("Current Studio reads refreshed.");
  }

  document.addEventListener("click", (event) => {
    const proseLink = event.target.closest(".prose a");
    if (proseLink && openSourceLink(proseLink)) { event.preventDefault(); return; }
    const sectionLink = event.target.closest("[data-section-link]");
    if (sectionLink) { event.preventDefault(); navigate(sectionLink.dataset.sectionLink, "overview"); return; }
    const target = event.target.closest("button"); if (!target) return;
    const action = target.dataset.action;
    if (action === "toggle-mobile-nav") { const panel = one("#mobile-navigation"); panel.hidden = !panel.hidden; target.setAttribute("aria-expanded", String(!panel.hidden)); }
    if (action === "navigate-section") navigate(target.dataset.section, "overview");
    if (action === "navigate-view") navigate(target.dataset.section ?? state.section, target.dataset.view);
    if (action === "open-source") openSource(target.dataset.source, target.dataset.heading);
    if (action === "open-heading") { state.selectedHeading = target.dataset.heading; updateUrl(); renderApp(); }
    if (action === "open-model") openModel(target.dataset.model);
    if (action === "open-tool") openTool(target.dataset.tool);
    if (action === "open-table") openTable(target.dataset.table);
    if (action === "open-entity") void openEntity(target.dataset.entity);
    if (action === "open-activity") void openActivity(target.dataset.activity);
    if (action === "open-development-status") { state.section = "game"; state.view = "development"; state.developmentStatus = target.dataset.status; updateUrl(); renderApp(); focusContent(); }
    if (action === "clear-development-status") { state.developmentStatus = null; updateUrl(); renderApp(); }
    if (action === "clear-entity") { state.selectedEntityId = null; state.entityDetail = null; updateUrl(); renderApp(); focusContent(); }
    if (action === "clear-activity") { state.selectedActivityId = null; state.activityDetail = null; updateUrl(); renderApp(); focusContent(); }
    if (action === "close-copy-fallback") closeCopyFallback();
    if (action === "copy-reference") void copyText(target.dataset.reference ?? currentReference());
    if (action === "copy-link") void copyText(location.href, "Link copied.");
    if (action === "copy-value") void copyText(target.dataset.value);
    if (action === "load-more") void loadMore(target.dataset.kind);
    if (action === "open-direct-entity") {
      const value = one("#direct-entity-id")?.value.trim(); if (!UUID.test(value)) toast("Enter one complete Entity UUID."); else void openEntity(value.toLowerCase());
    }
    if (action === "refresh") void refresh();
  });

  document.addEventListener("input", (event) => {
    const kind = event.target.dataset.filter; if (!kind) return; state.filter[kind] = event.target.value; renderApp(); one(`#filter-${kind}`)?.focus();
  });

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && !one("[data-copy-fallback]").hidden) closeCopyFallback();
  });

  window.addEventListener("popstate", () => {
    parseLocation(); renderApp();
    if (state.selectedEntityId) void openEntity(state.selectedEntityId, "replace");
    if (state.selectedActivityId) void openActivity(state.selectedActivityId, "replace");
  });

  parseLocation(); updateUrl("replace"); renderHeader(); renderSidebar(); void loadInitial();
})();
