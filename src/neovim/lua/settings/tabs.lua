--[[

    Editor Tabs

--]]


-- Creation
vim.api.nvim_set_keymap('n', '<Leader>T', ':-tabnew<CR>', {noremap = true, silent = true})
vim.api.nvim_set_keymap('n', '<Leader>t',  ':tabnew<CR>', {noremap = true, silent = true})


-- Navigation
vim.api.nvim_set_keymap('n', '<Leader>,', ':tabprevious<CR>', {noremap = true, silent = true})
vim.api.nvim_set_keymap('n', '<Leader>.', ':tabnext<CR>',     {noremap = true, silent = true})


-- Reordering
vim.api.nvim_set_keymap('n', '<Leader><', ':tabmove -1<CR>', {noremap = true, silent = true})
vim.api.nvim_set_keymap('n', '<Leader>>', ':tabmove +1<CR>', {noremap = true, silent = true})
