--[[

    Mason Language Suite Manager


    References:

        :help LSP
        :help mason.nvim
        :help mason-lspconfig.nvim
        :help lspconfig.txt
        :help nvim-cmp
        :help cmp-nvim-lsp
        :help neodev.txt

        https://github.com/williamboman/mason.nvim
        https://github.com/williamboman/mason-lspconfig.nvim
        https://github.com/neovim/nvim-lspconfig
        https://github.com/hrsh7th/cmp-nvim-lsp
        https://github.com/folke/neodev.nvim

--]]


return {
    'williamboman/mason.nvim',
    enabled = true,
    dependencies = {
        'williamboman/mason-lspconfig.nvim',
        'neovim/nvim-lspconfig',
        'hrsh7th/cmp-nvim-lsp',
        'folke/neodev.nvim',
    },
    config = function ()
        -- Dependency Modules
        local mason           = require('mason')
        local cmp_nvim_lsp    = require('cmp_nvim_lsp')
        local mason_lspconfig = require('mason-lspconfig')
        local lspconfig       = require('lspconfig')
        local neodev          = require('neodev')

        -- LSP Settings
	    local lsp_settings = {
            vimls         = {},
            lua_ls        = {},
            bashls        = {},
            clangd        = {},
            rust_analyzer = {},
	    }

        -- Capabilities
    	local basic_capabilities    =
            cmp_nvim_lsp.default_capabilities()
        local enhanced_capabilities =
            vim.lsp.protocol.make_client_capabilities()
        -- print('   Basic: ' .. table.unpack(basic_capabilities))
        -- print('Enhanced: ' .. table.unpack(enhanced_capabilities))
    	local capabilities = cmp_nvim_lsp.default_capabilities(
            enhanced_capabilities
        )
        -- print('  Result: ' .. table.unpack(capabilities))


        --[[

            Note that the order of the following setup calls is important:

                1)  mason
                2)  neodev (just needs to be before lspconfig)
                3)  mason_lspconfig
                4)  lspconfig

        --]]


        -- Mason
	    mason.setup()

        -- Neodev
        neodev.setup()

        -- Mason LSP Config
    	mason_lspconfig.setup({
            ensure_installed = vim.tbl_keys(lsp_settings),
	    })

        -- LSP Config
	    mason_lspconfig.setup_handlers({
	        function(lsp_server)
                lspconfig[lsp_server].setup({
		            capabilities = capabilities,
                    settings = lsp_settings[lsp_server],
		        })
            end,
	    })
    end,
}
