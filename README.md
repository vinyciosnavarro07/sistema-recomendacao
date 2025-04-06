# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
Este projeto tem como objetivo a criação de um sistema de busca eficiente e seguro para o catálogo de produtos da MegaStore. A solução permite aos clientes pesquisar produtos por nome, marca ou categoria, retornando resultados relevantes e organizados. O sistema também inclui uma lógica de recomendação baseada em similaridade de categoria.

## Tecnologias Utilizadas
- **Linguagem**: Rust
- **Crates (bibliotecas)**:
  - `unicode-normalization` - Normalização de strings e remoção de acentos
  - `std::collections::HashMap` - Estrutura principal de indexação
- **Ferramentas de teste**: Cargo test

## Instruções de Execução
1. Clone o repositório do projeto:
   ```bash
   git clone https://github.com/vinyciosnavarro07/sistema-recomendacao
   cd sistema-recomendacao
2. Compile o projeto:
    ```bash
    cargo build
3. Execute o sistema:
   ```bash
    cargo run
   
# Instruções de Execução dos Testes
## Para rodar os testes unitários e de integração:
  ```bash
    cargo test
````

# Exemplos de Uso
## Digite um termo de busca:
```bash
Digite o termo de busca (ou 'sair' para encerrar): 
> smartphone
````

## Resultado esperado:
```bash
🔎 Produtos encontrados:
- [1] Smartphone Galaxy S21 (Samsung, Eletrônicos)
- [2] Smartphone iPhone 13 (Apple, Eletrônicos)
````
## Arquitetura do Sistema
- `main.rs`: Ponto de entrada do sistema, coordena busca e carregamento de produtos.
- `produto.rs`: Define a estrutura `Produto`.
- `index.rs`: Implementa a lógica de indexação e busca com `HashMap`.
- `recomendacao.rs`: Implementa recomendações com base em similaridade por categoria.

## Algoritmos e Estruturas de Dados Utilizados
- **HashMap<String, Vec<u64>**: Estrutura para indexar termos por produto.
- **Grafo de recomendação**: Conecta produtos por categoria para oferecer sugestões relevantes.
- **Tokenização e normalização**: Processos essenciais para garantir buscas eficientes, mesmo com variações nos termos inseridos pelos usuários.

## Considerações sobre Desempenho e Escalabilidade
- A busca tem complexidade próxima de **O(1)** para acesso ao `HashMap`.
- O sistema se mostrou responsivo mesmo com dezenas de produtos.
- Estrutura projetada para escalabilidade, permitindo integração com banco de dados ou cache em ambientes de produção.

## Licença
Este projeto está licenciado sob a **Licença MIT**.
