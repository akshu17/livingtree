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

WebUI.openBrowser(GlobalVariable.URL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_PARENT)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.verifyElementPresent(findTestObject('LivingTree/MyProfilePageLink/ParentProfileDropDownIcon'), 0)

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/ParentProfileDropDownIcon'))

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/MyProfile'))

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/EditProfilePreferencesButton'), FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementVisible(findTestObject('LivingTree/MyProfilePageLink/ProfilePreferencesPage'))

WebUI.closeBrowser()

WebUI.delay(2)

WebUI.openBrowser(GlobalVariable.URL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_TEACHER)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.verifyElementPresent(findTestObject('LivingTree/MyProfile/TeacherDropDown'), 0)

WebUI.click(findTestObject('LivingTree/MyProfile/TeacherDropDown'))

WebUI.click(findTestObject('LivingTree/MyProfile/TeacherMyProfile'))

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/EditProfilePreferencesButton'), FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementVisible(findTestObject('LivingTree/MyProfilePageLink/ProfilePreferencesPage'))

WebUI.closeBrowser()

WebUI.delay(2)

WebUI.openBrowser(GlobalVariable.URL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_STUDENT)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.verifyElementPresent(findTestObject('LivingTree/MyProfile/DropDownOption'), 0)

WebUI.click(findTestObject('LivingTree/MyProfile/DropDownOption'))

WebUI.click(findTestObject('LivingTree/MyProfile/TeacherMyProfile'))

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/EditProfilePreferencesButton'), FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementVisible(findTestObject('LivingTree/MyProfilePageLink/ProfilePreferencesPage'))

WebUI.closeBrowser()

WebUI.delay(2)

WebUI.openBrowser(GlobalVariable.URL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_PRINCIPLE)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.verifyElementPresent(findTestObject('LivingTree/MyProfile/DropDownOption'), 0)

WebUI.click(findTestObject('LivingTree/MyProfile/DropDownOption'))

WebUI.click(findTestObject('LivingTree/MyProfile/TeacherMyProfile'))

WebUI.click(findTestObject('LivingTree/MyProfilePageLink/EditProfilePreferencesButton'), FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementVisible(findTestObject('LivingTree/MyProfilePageLink/ProfilePreferencesPage'))

WebUI.closeBrowser()

