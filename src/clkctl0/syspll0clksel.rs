#[doc = "Register `SYSPLL0CLKSEL` reader"]
pub type R = crate::R<Syspll0clkselSpec>;
#[doc = "Register `SYSPLL0CLKSEL` writer"]
pub type W = crate::W<Syspll0clkselSpec>;
#[doc = "System PLL Clock Source Selection. . .\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: SFRO Clock."]
    SfroClk = 0,
    #[doc = "1: SYSXTALIN Clock."]
    SysxtalClk = 1,
    #[doc = "2: FFRO Clock Divided by 2."]
    FfroDiv2 = 2,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    None = 7,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - System PLL Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::SfroClk),
            1 => Some(Sel::SysxtalClk),
            2 => Some(Sel::FfroDiv2),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn is_sfro_clk(&self) -> bool {
        *self == Sel::SfroClk
    }
    #[doc = "SYSXTALIN Clock."]
    #[inline(always)]
    pub fn is_sysxtal_clk(&self) -> bool {
        *self == Sel::SysxtalClk
    }
    #[doc = "FFRO Clock Divided by 2."]
    #[inline(always)]
    pub fn is_ffro_div_2(&self) -> bool {
        *self == Sel::FfroDiv2
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - System PLL Clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn sfro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SfroClk)
    }
    #[doc = "SYSXTALIN Clock."]
    #[inline(always)]
    pub fn sysxtal_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SysxtalClk)
    }
    #[doc = "FFRO Clock Divided by 2."]
    #[inline(always)]
    pub fn ffro_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FfroDiv2)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - System PLL Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System PLL Clock Source Selection. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<Syspll0clkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "system pll0 clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`syspll0clksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspll0clksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspll0clkselSpec;
impl crate::RegisterSpec for Syspll0clkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspll0clksel::R`](R) reader structure"]
impl crate::Readable for Syspll0clkselSpec {}
#[doc = "`write(|w| ..)` method takes [`syspll0clksel::W`](W) writer structure"]
impl crate::Writable for Syspll0clkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPLL0CLKSEL to value 0x07"]
impl crate::Resettable for Syspll0clkselSpec {
    const RESET_VALUE: u32 = 0x07;
}
