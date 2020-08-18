import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/RymsGroupBox'), 0)

WebUI.click(findTestObject('LivingTree/CalendarLink/InviteParent(s)usinggroupcode'))

WebUI.delay(1)

WebUI.click(findTestObject('LivingTree/CalendarLink/ParentsGroupButtonClose'))

WebUI.click(findTestObject('LivingTree/CalendarLink/RymsAddMember'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/RymsGroupBox'), 0)

WebUI.click(findTestObject('LivingTree/CalendarLink/InviteParent(s)viae-mail'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/ParentInviteBox'), 0)

WebUI.click(findTestObject('LivingTree/CalendarLink/InviteParentviaButtonClose'))

WebUI.click(findTestObject('LivingTree/CalendarLink/RymsAddMember'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/RymsGroupBox'), 0)

WebUI.click(findTestObject('LivingTree/CalendarLink/InviteStudent(s)usinggroupcode'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/ManagegroupcodeRymsParents'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/ParentGroupCodePage'), 0)

WebUI.delay(0)

WebUI.click(findTestObject('LivingTree/CalendarLink/ParentsGroupButtonClose'))

WebUI.click(findTestObject('LivingTree/CalendarLink/RymsAddMember'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/RymsGroupBox'), 0)

WebUI.click(findTestObject('LivingTree/CalendarLink/InviteStudent(s)viaemail'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/ParentInviteBox'), 0)

WebUI.delay(1)

WebUI.click(findTestObject('LivingTree/CalendarLink/InviteParentviaButtonClose'))

WebUI.closeBrowser()
