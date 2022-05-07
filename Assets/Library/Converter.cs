using UnityEngine;
using System.Collections;

static public class Converter {
    //public const float AuToMeters = 149597870700;
    public const double AuToMetersDouble = 149597.870700f / 2d;
    public const double MetersToAuDouble = 1d / AuToMeters;

    public const float AuToMeters = (float) AuToMetersDouble;
    public const float MetersToAu = (float) MetersToAuDouble;
}
