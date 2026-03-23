return {
  "epwalsh/obsidian.nvim",
  version = "*",
  lazy = true,
  ft = "markdown",
  dependencies = {
    "nvim-lua/plenary.nvim",
  },
  opts = {
    workspaces = {
      {
        name = "amphora",
        path = "~/amphora",
      },
    },

    daily_notes = {
      folder = function()
        local months = {
          "Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho",
          "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro",
        }
        local month = tonumber(os.date("%m"))
        local year = os.date("%Y")
        return "Daily Notes/" .. year .. "/" .. months[month]
      end,
      date_format = "%d-%m-%Y",
      template = nil,
    },

    completion = {
      nvim_cmp = false,
      min_chars = 2,
    },

    new_notes_location = "current_dir",

    preferred_link_style = "wiki",

    picker = {
      name = "snacks.nvim",
    },

    -- UI desabilitado: render-markdown.nvim (via lang.markdown extra) cuida do rendering
    ui = { enable = false },

    mappings = {
      -- Seguir link sob o cursor
      ["gf"] = {
        action = function() return require("obsidian").util.gf_passthrough() end,
        opts = { noremap = false, expr = true, buffer = true },
      },
      -- Toggle checkbox
      ["<leader>ch"] = {
        action = function() return require("obsidian").util.toggle_checkbox() end,
        opts = { buffer = true },
      },
      -- Hover: preview flutuante do [[link]] sob o cursor
      ["K"] = {
        action = function()
          local line = vim.api.nvim_get_current_line()
          local col = vim.api.nvim_win_get_cursor(0)[2] + 1
          -- Extrai [[nome]] ou [[nome|alias]] na posição do cursor
          local link = nil
          for s, inner, e in line:gmatch("()%[%[(.-)%]%]()") do
            if col >= s and col <= e then
              link = inner:match("^([^|#]+)") -- ignora alias e âncoras
              break
            end
          end
          if not link then return end
          link = vim.trim(link)
          -- Resolve o arquivo no vault
          local vault = vim.fn.expand("~/amphora")
          local result = vim.fn.systemlist(
            "find " .. vim.fn.shellescape(vault) ..
            " -iname " .. vim.fn.shellescape(link .. ".md") ..
            " -not -path '*/Templates/*' 2>/dev/null | head -1"
          )
          local path = result[1]
          if not path or path == "" then
            vim.notify("Nota não encontrada: " .. link, vim.log.levels.WARN)
            return
          end
          -- Lê o conteúdo e exibe numa janela flutuante
          local lines = vim.fn.readfile(path)
          -- Remove frontmatter
          if lines[1] == "---" then
            for i = 2, #lines do
              if lines[i] == "---" then
                lines = vim.list_slice(lines, i + 1)
                break
              end
            end
          end
          -- Limita a 30 linhas na preview
          if #lines > 30 then
            lines = vim.list_slice(lines, 1, 30)
            table.insert(lines, "…")
          end
          local buf = vim.api.nvim_create_buf(false, true)
          vim.api.nvim_buf_set_lines(buf, 0, -1, false, lines)
          vim.bo[buf].filetype = "markdown"
          local width = math.min(80, vim.o.columns - 4)
          local height = math.min(#lines, 20)
          vim.api.nvim_open_win(buf, false, {
            relative = "cursor",
            row = 1, col = 0,
            width = width, height = height,
            style = "minimal",
            border = "rounded",
            title = " " .. link .. " ",
            title_pos = "center",
          })
        end,
        opts = { buffer = true },
      },
    },

    attachments = {
      img_folder = "Assets",
    },
  },
}
