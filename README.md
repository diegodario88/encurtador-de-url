### Encurtador de URL em Rust

Este projeto apresenta um encurtador de URL simples implementado em Rust. O objetivo principal é fornecer uma ferramenta
prática para encurtar URLs longas e complexas, facilitando o compartilhamento em diversos contextos.

## Funcionalidades

- Encurtar URLs longas em URLs mais curtas e fáceis de lembrar.
- Autenticação usando "x-api-key:ijhbKDJBKD".
- Monitorar o acesso às URLs encurtadas.
- **Observabilidade com Prometheus**: Colete métricas sobre o uso do encurtador de URL para análise e monitoramento.

## Instalação

Para instalar e executar o projeto, siga as etapas a seguir:

1. **Clonar**: Clone o repositório

```bash
git clone https://github.com/diegodario88/encurtador-de-url
```

2. **Navegar para o Diretório**: Entre no diretório do projeto:

```bash
cd encurtador-de-url
```

3. **Endereço de IP**: Descubra o endereço de IP e substitua nos campos `extra_hosts:` do arquivo docker.compose.yml
   > No windows
   ```bash
   Get-NetIPAddress -AddressFamily IPv4 | Where-Object {$_.InterfaceAlias -eq 'Ethernet'} | Select-Object -ExpandProperty IPAddress
   ```
   > No linux
   ```bash
   ip addr show | grep docker
   ```

4. **Executar o Projeto**: Execute o seguinte comando para compilar e executar o aplicativo:

```bash
docker compose up
```

5. **Acessar as Métricas**: Abra o navegador o sguinte endereço:

```
http://localhost:3030
```

## Observabilidade com Prometheus

O projeto integra o Prometheus para coletar métricas sobre o uso do encurtador de URL. As métricas disponíveis incluem:

- `encurtador_url_requests_total`: Contador total de solicitações de encurtamento de URL.
- `encurtador_url_requests_success`: Contador de solicitações de encurtamento de URL bem-sucedidas.
- `encurtador_url_requests_fail`: Contador de solicitações de encurtamento de URL falhadas.
- `encurtador_url_redirects_total`: Contador total de redirecionamentos para URLs encurtadas.

## Considerações Finais

Este projeto oferece uma solução básica para encurtar URLs em Rust, com a adição de observabilidade através do Prometheus para auxiliar na análise e monitoramento do uso do encurtador. Você pode aprimorá-lo adicionando novas funcionalidades, como personalização de URLs, integração com APIs de terceiros e implementação de mecanismos de segurança mais robustos.

## Contribuições

Se você deseja contribuir para o desenvolvimento deste projeto, sinta-se à vontade para enviar pull requests com suas sugestões e melhorias.
