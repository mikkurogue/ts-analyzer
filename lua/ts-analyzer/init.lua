---@diagnostic disable: undefined-global
local M = {}
local v = vim

---@class Config
---@field attach boolean Auto-attach to LSP servers (default: true)
---@field servers string[] LSP server names to translate diagnostics for

local root = debug.getinfo(1, "S").source:sub(2):match("(.*/)")
root = root:match("(.*/)") -- move up 1 dir

local bin = root .. "target/release/ts-analyzer"

-- All of this is stolen from the goat @dmmulroy
local function get_lsp_client_name_by_id(id)
  local client = v.lsp.get_client_by_id(id)
  local name = client and client.name or "unknown"
  return name
end

-- All of this is stolen from the goat @dmmulroy
function M.setup(opts)
  local original_diagnostics = v.lsp.handlers["textDocument/publishDiagnostics"]

  v.lsp.handlers["textDocument/publishDiagnostics"] = function(err, result, ctx, config)
    if result and result.diagnostics then
      local client_name = get_lsp_client_name_by_id(ctx.client_id)

      if v.tbl_contains(opts.servers, client_name) then
        for _, diag in ipairs(result.diagnostics) do
          if diag.message then
            -- TODO: call ts-analyzer to translate message
            diag.message = "[TS Analyzer] " .. "TODO"
          end
        end
      end
    end
    return original_diagnostics(err, result, ctx, config)
  end
end

-- @type Config
local DEF_OPTS = {
  -- auto attach to the lsp
  attach = true,
  -- list of lsp server names to attach to
  -- only supporting these two for now cause im lazy and dont want to deal with testing
  servers = {
    "ts_ls",
    "vtsls"
  }
}

-- @type Config
M.config = DEF_OPTS


function M.setup(opts)
  opts = ops or {}

  M.config = v.tbl_deep_extend("force", DEF_OPTS, opts)

  if M.config.attach then
    local diag_cfg = { servers = M.config.servers }

    require("ts-analyzer").setup(diag_cfg)
  end
end

return M
