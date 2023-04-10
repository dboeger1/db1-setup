--[[

    Editor Windows

--]]


-- Splitting
vim.opt.splitbelow = true
vim.opt.splitright = true

vim.api.nvim_set_keymap('n', '<Leader>w', ':vnew<CR>', {noremap = true, silent = true})
vim.api.nvim_set_keymap('n', '<Leader>W',  ':new<CR>', {noremap = true, silent = true})


-- Navigation
vim.api.nvim_set_keymap('n', '<Leader>/', '<C-W>w', {noremap = true, silent = true})


-- Resizing
vim.api.nvim_set_keymap('n', '<Leader>-', '<C-W>=',       {noremap = true, silent = true})
vim.api.nvim_set_keymap('n', '<Leader>=', '<C-W>|<C-W>_', {noremap = true, silent = true})

vim.api.nvim_set_keymap('n', '<Leader>[', ':vertical resize -8<CR>', {noremap = true, silent = true})
vim.api.nvim_set_keymap('n', '<Leader>]', ':vertical resize +8<CR>', {noremap = true, silent = true})
vim.api.nvim_set_keymap('n', '<Leader>{', ':resize -4<CR>', {noremap = true, silent = true})
vim.api.nvim_set_keymap('n', '<Leader>}', ':resize +4<CR>', {noremap = true, silent = true})


-- Tab Conversion
vim.api.nvim_set_keymap('n', '<Leader>?', '<C-W>T', {noremap = true, silent = true})
