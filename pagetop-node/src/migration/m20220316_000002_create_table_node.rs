use pagetop::prelude::*;

#[derive(Iden)]
enum Node {
    Table,              // node: The base table for nodes.

    Nid,                // The primary identifier for a node.
    Vid,                // The current NodeRevision.vid version identifier.
    Type,               // The NodeType.type of this node.
    Language,           // The {languages}.language of this node.
    Title,              // The title of this node, always treated as non-markup plain text.
    Uid,                // The User.uid that owns this node; initially, this is the user that
                        // created it.
    Status,             // Boolean indicating whether the node is published (visible to
                        // non-administrators).
    Created,            // The Unix timestamp when the node was created.
    Changed,            // The Unix timestamp when the node was most recently saved.
    Comment,            // Whether comments are allowed on this node: 0 = no, 1 = closed (read
                        // only), 2 = open (read/write).
    Promote,            // Boolean indicating whether the node should be displayed on the front
                        // page.
    Sticky,             // Boolean indicating whether the node should be displayed at the top of
                        // lists in which it appears.
    Tnid,               // The translation set id for this node, which equals the node id of the
                        // source post in each set.
    Translate,          // A boolean indicating whether this translation page needs to be updated.
}

pub_migration!(Migration);

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
            .table(Node::Table)
            .if_not_exists()
            .col(ColumnDef::new(Node::Nid)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
            )
            .col(ColumnDef::new(Node::Vid)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Type)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Language)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Title)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Uid)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Status)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Created)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Changed)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Comment)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Promote)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Sticky)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Tnid)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(Node::Translate)
                .string()
                .not_null()
            )
            .to_owned()
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop()
            .table(Node::Table)
            .to_owned()
        )
        .await
    }
}
