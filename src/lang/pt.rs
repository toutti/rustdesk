lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("desk_tip", "Sua área de trabalho está acessível através deste ID e senha."),
        ("connecting_status", "Conectando-se à rede RustDesk..."),
        ("not_ready_status", "Não está pronto. Por favor, verifique sua conexão"),
        ("id_change_tip", "Somente caracteres a-z, A-Z, 0-9 e _ (traço baixo) são permitidos. A primeira letra deve ser a-z, A-Z. Comprimento entre 6 e 16."),
        ("install_tip", "Devido o UAC, RustDesk pode não funcionar corretamente do lado remoto em alguns casos. Para evitar o UAC, por favor clique no botão abaixo para instalar o RustDesk no sistema."),
        ("config_acc", "Para controle da sua área de trabalho remota, você precisa conceder ao RustDesk \"Acessibilidade\" permissões."),
        ("config_screen", "Para acesso da sua área de trabalho remota, você precisa conceder ao RustDesk \"Gravação de tela\" permissões."),
        ("agreement_tip", "Ao iniciar a instalação, você aceita o contrato de licença."),
        ("not_close_tcp_tip", "Não feche esta janela enquanto estiver usando o túnel"),
        ("setup_server_tip", "Para uma conexão mais rápida, configure seu próprio servidor"),
        ("Auto Login", "Login automático (Só é válido se você definir \"Bloquear após o término da sessão\")"),
        ("whitelist_tip", "Somente IP's na lista de permitidos pode me acessar"),
        ("whitelist_sep", "Separados por vírgula, ponto e vírgula, espaços ou nova linha"),
        ("Wrong credentials", "Nome de usuário ou senha incorretos"),
        ("invalid_http", "Devemos começar com http:// ou https://"),
        ("install_daemon_tip", "Para carregar na inicialização, você precisa instalar o serviço no sistema."),
    ].iter().cloned().collect();
}
