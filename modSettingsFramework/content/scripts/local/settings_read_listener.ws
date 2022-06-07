abstract class ISettingsReadListener
{
    //TODO argument that tells you where settings were updated (in main menu? in pause menu?)
    public function OnReadSettings() : void;
}