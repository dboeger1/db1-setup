--[[

    nvim-cmp Completion Engine


    References:

        :help nvim-cmp

        https://github.com/hrsh7th/nvim-cmp

--]]


return {
    'hrsh7th/nvim-cmp',
    enabled = true,

    dependencies = {
        'hrsh7th/cmp-nvim-lsp',
        'dcampos/nvim-snippy',
        'dcampos/cmp-snippy',
        'hrsh7th/cmp-path',
    },

    config = function()
        -- Dependency Modules
        local cmp     = require('cmp')
        local snippy  = require('snippy')

        -- Setup
        cmp.setup({
            snippet = {
                expand = function(args)
                    snippy.expand_snippet(args.body)
                end,
            },
            sources = {
                { name = 'nvim_lsp' },
                { name = 'snippy' },
                { name = 'path' },
            },
            mapping = cmp.mapping.preset.insert({
                ['<CR>']    = cmp.mapping.confirm(),
                ['<S-Tab>'] = cmp.mapping.select_prev_item(),
                ['<Tab>']   = cmp.mapping.select_next_item(),
                ['<C-K>']   = cmp.mapping.scroll_docs(-4),
                ['<C-J>']   = cmp.mapping.scroll_docs(4),
                ['<C-X>']   = cmp.mapping.abort(),
            }),
        })
    end
}
