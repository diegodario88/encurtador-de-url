### Encurtador de URL em Rust

Este projeto apresenta um encurtador de URL simples implementado em Rust. O objetivo principal é fornecer uma ferramenta prática para encurtar URLs longas e complexas, facilitando o compartilhamento em diversos contextos.

## Funcionalidades

- Encurtar URLs longas em URLs mais curtas e fáceis de lembrar.
- Personalizar o alias da URL encurtada (opcional).
- Gerar QR Codes para URLs encurtadas.
- Monitorar o acesso às URLs encurtadas (opcional).
- **Observabilidade com Prometheus**: Colete métricas sobre o uso do encurtador de URL para análise e monitoramento.

## Instalação

Para instalar e executar o projeto, siga as etapas a seguir:

1. **Instalar Rust**: Certifique-se de ter o Rust instalado em seu sistema. Você pode encontrar instruções de instalação em [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. **Clonar o Repositório**: Clone o repositório do projeto usando o comando:

```bash
git clone https://github.com/topics/encurtador-de-url
```

3. **Navegar para o Diretório**: Entre no diretório do projeto:

```bash
cd encurtador-url-rust
```

4. **Executar o Projeto**: Execute o seguinte comando para compilar e executar o aplicativo:

```bash
cargo run
```

## Uso

Ao executar o aplicativo, você será apresentado a uma interface de linha de comando. Digite a URL longa que deseja encurtar e pressione Enter. O aplicativo irá gerar uma URL encurtada e exibir na tela.

Opcionalmente, você pode personalizar o alias da URL encurtada. Para fazer isso, digite o alias desejado após a URL longa, separado por um espaço. Por exemplo:

```
encurtador-url-rust https://exemplodelinklongo meu-alias-personalizado
```

Para gerar um QR Code para a URL encurtada, adicione a flag `--qr` ao comando:

```bash
encurtador-url-rust https://exemplodelinklongo --qr
```

## Monitoramento de Acessos (Opcional)

O aplicativo pode ser configurado para monitorar o acesso às URLs encurtadas. Para ativar o monitoramento, adicione a flag `--monitor` ao comando e configure o banco de dados de acordo com as instruções na documentação.

## Observabilidade com Prometheus

O projeto integra o Prometheus para coletar métricas sobre o uso do encurtador de URL. As métricas disponíveis incluem:

- `encurtador_url_requests_total`: Contador total de solicitações de encurtamento de URL.
- `encurtador_url_requests_success`: Contador de solicitações de encurtamento de URL bem-sucedidas.
- `encurtador_url_requests_fail`: Contador de solicitações de encurtamento de URL falhadas.
- `encurtador_url_redirects_total`: Contador total de redirecionamentos para URLs encurtadas.

Para coletar essas métricas, você precisará executar um servidor Prometheus e configurar o raspador para coletar dados do endpoint do aplicativo. O endpoint padrão para métricas Prometheus é `http://localhost:8080/metrics`.

## Considerações Finais

Este projeto oferece uma solução básica para encurtar URLs em Rust, com a adição de observabilidade através do Prometheus para auxiliar na análise e monitoramento do uso do encurtador. Você pode aprimorá-lo adicionando novas funcionalidades, como personalização de URLs, integração com APIs de terceiros e implementação de mecanismos de segurança mais robustos.

## Contribuições

Se você deseja contribuir para o desenvolvimento deste projeto, sinta-se à vontade para enviar pull requests com suas sugestões e melhorias.
