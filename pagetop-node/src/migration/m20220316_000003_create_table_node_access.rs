use pagetop::prelude::*;

#[derive(Iden)]
enum NodeAccess {
    Table,              // node_access: Identifies which realm/grant pairs a user must possess in
                        // order to view, update, or delete specific nodes.

    Nid,                // The Node.nid this record affects.
    Gid,                // The grant ID a user must possess in the specified realm to gain this
                        // row's privileges on the node.
    Realm,              // The realm in which the user must possess the grant ID. Each node access
                        // node can define one or more realms.
    GrantView,          // Boolean indicating whether a user with the realm/grant pair can view this
                        // node.
    GrantUpdate,        // Boolean indicating whether a user with the realm/grant pair can edit this
                        // node.
    GrantDelete,        // Boolean indicating whether a user with the realm/grant pair can delete
                        // this node.
}

pub_migration!(Migration);

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
            .table(NodeAccess::Table)
            .if_not_exists()
            .col(ColumnDef::new(NodeAccess::Nid)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
            )
            .col(ColumnDef::new(NodeAccess::Gid)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(NodeAccess::Realm)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(NodeAccess::GrantView)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(NodeAccess::GrantUpdate)
                .string()
                .not_null()
            )
            .col(ColumnDef::new(NodeAccess::GrantDelete)
                .string()
                .not_null()
            )
            .to_owned()
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop()
            .table(NodeAccess::Table)
            .to_owned()
        )
        .await
    }
}
