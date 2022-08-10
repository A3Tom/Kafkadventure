using System.ComponentModel;

namespace KA.Domain.Enums;
public enum LochEnum
{
    [Description("Loch Ness")]
    LOCH_NESS = 0,

    [Description("Loch Lomand")]
    LOCH_LOMAND = 1,

    [Description("Loch Morar")]
    LOCH_MORAR = 2,

    [Description("Loch Tay")]
    LOCH_TAY = 3,

    [Description("Loch Awe")]
    LOCH_AWE= 4,
}
