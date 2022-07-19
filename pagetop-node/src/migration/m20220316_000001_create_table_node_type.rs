use pagetop::prelude::*;

#[derive(Iden)]
enum NodeType {
    Table,              // node_type: Stores information about all defined Node types.

    Type,               // The machine-readable name of this type.
    Name,               // The human-readable name of this type.
    Description,        // Descripción breve del tipo.
    Help,               // Help information shown to the user when creating a Node of this type.
    HasTitle,           // Boolean indicating whether this type uses the Node.Title field.
    TitleLabel,         // The label displayed for the title field on the edit form.
    Custom,             // A boolean indicating whether this type is defined by a module (FALSE) or
                        // by a user via Add content type (TRUE).
    Locked,             // A boolean indicating whether the administrator can change the machine
                        // name of this type.
    Disabled,           // A boolean indicating whether the node type is disabled.
    OrigType,           // The original machine-readable name of this node type, this may be
                        // different from the current type name if the locked field is 0.
}

pub_migration!(Migration);

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NodeType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NodeType::Type)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NodeType::Name).string().not_null())
                    .col(ColumnDef::new(NodeType::Description).string().not_null())
                    .col(ColumnDef::new(NodeType::Help).string().not_null())
                    .col(ColumnDef::new(NodeType::HasTitle).string().not_null())
                    .col(ColumnDef::new(NodeType::TitleLabel).string().not_null())
                    .col(ColumnDef::new(NodeType::Custom).string().not_null())
                    .col(ColumnDef::new(NodeType::Locked).string().not_null())
                    .col(ColumnDef::new(NodeType::Disabled).string().not_null())
                    .col(ColumnDef::new(NodeType::OrigType).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NodeType::Table).to_owned())
            .await
    }
}
