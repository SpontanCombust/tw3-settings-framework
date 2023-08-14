exec function monster_of_the_week()
{
    var settings: MonsterOfTheWeekSettings;
    var area: EAreaName;

    area = theGame.GetCommonMapManager().GetCurrentArea();

    switch(area) 
    {
    case AN_NMLandNovigrad:
    case AN_Velen:
        theGame.GetGuiManager().ShowNotification("Current MOTW in Velen and Novigrad: " + settings.monsters.noMansLand);
        break;
    case AN_Skellige_ArdSkellig:
        theGame.GetGuiManager().ShowNotification("Current MOTW in Skellige: " + settings.monsters.skellige);
        break;
    case AN_Kaer_Morhen:
        theGame.GetGuiManager().ShowNotification("Current MOTW in Kaer Morhen: " + settings.monsters.kaerMorhen);
        break;
    case (EAreaName)AN_Dlc_Bob:
        theGame.GetGuiManager().ShowNotification("Current MOTW in Toussaint: " + settings.monsters.toussaint);
        break;
    default:
        theGame.GetGuiManager().ShowNotification("Current area not supported");
    }
}