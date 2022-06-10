abstract class ISettingsMaster
{
    public function ReadSettings() : void {}
    public function WriteSettings() : void {}

    // listener methods
    public function OnReadSettings() : void {}
    public function OnWriteSettings() : void {}
}