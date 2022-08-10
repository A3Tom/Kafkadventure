using System.ComponentModel;

namespace KA.Domain.Extensions;
public static class EnumExtensions
{
    public static string GetDescription<T>(this T value)
    {
       var attribute = typeof(T).GetField(value.ToString())
            .GetCustomAttributes(typeof(DescriptionAttribute), false)
            .SingleOrDefault();

        return attribute == null ?
            null :
            ((DescriptionAttribute)attribute).Description;
    }
}
