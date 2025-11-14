impl :: bincode :: Encode for BackupMetadata
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.files, encoder) ?; core :: result
        :: Result :: Ok(())
    }
}