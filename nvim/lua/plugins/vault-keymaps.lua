-- Vault keymaps — amphora
-- <leader>od — abrir daily note de hoje
-- <leader>ov — abrir vault
-- <leader>ot — adicionar task na daily note de hoje

local vault_months = {
  "Janeiro","Fevereiro","Março","Abril","Maio","Junho",
  "Julho","Agosto","Setembro","Outubro","Novembro","Dezembro",
}

vim.keymap.set("n", "<leader>od", function()
  local m = tonumber(os.date("%m"))
  local path = vim.fn.expand("~/amphora/Daily Notes/" .. os.date("%Y") .. "/" .. vault_months[m] .. "/" .. os.date("%d-%m-%Y") .. ".md")
  vim.cmd("edit " .. vim.fn.fnameescape(path))
end, { desc = "Open today's daily note" })

vim.keymap.set("n", "<leader>ov", function()
  vim.cmd("edit " .. vim.fn.expand("~/amphora"))
end, { desc = "Open vault" })

vim.keymap.set("n", "<leader>ot", function()
  local m = tonumber(os.date("%m"))
  local path = vim.fn.expand("~/amphora/Daily Notes/" .. os.date("%Y") .. "/" .. vault_months[m] .. "/" .. os.date("%d-%m-%Y") .. ".md")
  local task = vim.fn.input("Task: ")
  if task ~= "" then
    local f = io.open(path, "a")
    if f then f:write("- [ ] #task " .. task .. "\n"); f:close() end
    vim.notify("✓ task → " .. os.date("%d-%m-%Y"), vim.log.levels.INFO)
  end
end, { desc = "Add task to today's daily note" })

return {}
