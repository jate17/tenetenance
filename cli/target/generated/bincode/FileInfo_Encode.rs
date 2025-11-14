impl :: bincode :: Encode for FileInfo
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.relative_path, encoder) ?; ::
        bincode :: Encode :: encode(&self.hash, encoder) ?; :: bincode ::
        Encode :: encode(&self.timestamp_modified, encoder) ?; :: bincode ::
        Encode :: encode(&self.timestamp_backup, encoder) ?; :: bincode ::
        Encode :: encode(&self.deleted, encoder) ?; core :: result :: Result
        :: Ok(())
    }
}