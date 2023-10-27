use pagetop::prelude::*;

#[rustfmt::skip]
#[derive(Iden)]
enum NodeRevision {
    Table,              // node_revision: Stores information about each saved version of a Node.

    Nid,                // The Node this version belongs to.
    Vid,                // The primary identifier for this version.
    Uid,                // The User.uid that created this version.
    Title,              // The title of this version.
    Log,                // The log entry explaining the changes in this version.
    Timestamp,          // A Unix timestamp indicating when this version was created.
    Status,             // Boolean indicating whether the node (at the time of this revision) is
                        // published (visible to non-administrators).
    Comment,            // Whether comments are allowed on this node (at the time of this revision):
                        // 0 = no, 1 = closed (read only), 2 = open (read/write).
    Promote,            // Boolean indicating whether the node (at the time of this revision) should
                        // be displayed on the front page.
    Sticky,             // Boolean indicating whether the node (at the time of this revision) should
                        // be displayed at the top of lists in which it appears.
}

new_migration!(Migration);

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NodeRevision::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NodeRevision::Nid)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NodeRevision::Vid).string().not_null())
                    .col(ColumnDef::new(NodeRevision::Uid).string().not_null())
                    .col(ColumnDef::new(NodeRevision::Title).string().not_null())
                    .col(ColumnDef::new(NodeRevision::Log).string().not_null())
                    .col(ColumnDef::new(NodeRevision::Timestamp).string().not_null())
                    .col(ColumnDef::new(NodeRevision::Status).string().not_null())
                    .col(ColumnDef::new(NodeRevision::Comment).string().not_null())
                    .col(ColumnDef::new(NodeRevision::Promote).string().not_null())
                    .col(ColumnDef::new(NodeRevision::Sticky).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NodeRevision::Table).to_owned())
            .await
    }
}
